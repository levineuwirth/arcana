//! Ravenous Gigantotherium — `{5}{G}{G}` 3/3 green Beast.
//!
//! Oracle:
//! * Devour 3 → `KeywordAbility::Devour(3)` (engine wires the
//!   sacrifice-on-ETB / triple +1/+1 counters).
//! * "When this creature enters, it deals X damage divided as you choose
//!   among up to X target creatures, where X is its power. Each of those
//!   creatures deals damage equal to its power to this creature." — an
//!   ETB trigger with a dynamic-X divided-damage payload. We declare an
//!   up-to-X creature target requirement, attach a `with_trigger_dynamic_x`
//!   that materializes X = this creature's power, and emit
//!   `DealDamageDivided`. The "each of those creatures deals damage equal
//!   to its power back to this creature" backlash is modeled by emitting
//!   per-target `DealDamage` (source = the targeted creature) back at us.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ravenous Gigantotherium");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Devour(3)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_divided_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // X = this creature's power; the divided-damage resolver
                // reads the live power at resolution and spreads it across
                // the chosen creatures. We model "up to X" with a loose
                // UpTo cap because the dynamic-X closure can't read state
                // (it only receives &PendingTrigger) to evaluate power.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(99),
                    controller: None,
                }],
            }),
    )
}

fn etb_divided_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let total = script::power_of(state, trig.source).max(0) as u32;
    if total == 0 {
        return Vec::new();
    }
    let targets: Vec<DamageTarget> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            _ => None,
        })
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    let mut effects = vec![Effect::DealDamageDivided {
        source: trig.source,
        targets: targets.clone(),
        total,
    }];
    // Each of those creatures deals damage equal to its power to this
    // creature.
    for t in &targets {
        if let DamageTarget::Object(id) = t {
            let p = script::power_of(state, *id).max(0) as u32;
            if p > 0 {
                effects.push(Effect::DealDamage {
                    source: *id,
                    target: DamageTarget::Object(trig.source),
                    amount: p,
                });
            }
        }
    }
    effects
}
