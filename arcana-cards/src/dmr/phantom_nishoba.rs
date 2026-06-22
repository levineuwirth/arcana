//! Phantom Nishoba — `{5}{G}{W}` 0/0 Cat Beast Spirit.
//!
//! * Trample.
//! * This creature enters with seven +1/+1 counters on it. (ETB
//!   trigger placing the counters.)
//! * Whenever this creature deals damage, you gain that much life.
//!   (DamageDealt → GainLife by `damage_amount`. The engine's
//!   `DamageDealt` source filter cannot be restricted to exactly this
//!   object, so it is approximated by "a creature you control" — see
//!   GAP.)
//! * "If damage would be dealt to this creature, prevent that damage.
//!   Remove a +1/+1 counter from this creature." — a damage-replacement
//!   shield that consumes a counter per prevention; not expressible with
//!   the demonstrated primitives (no self-replacement-with-counter-cost
//!   hook). GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phantom Nishoba");
    let cat = reg.interner_mut().intern("Cat");
    let beast = reg.interner_mut().intern("Beast");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(beast);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "If damage would be dealt to this creature, prevent that
    // damage. Remove a +1/+1 counter from this creature." — a damage-
    // replacement that spends a counter per prevention; no demonstrated
    // self-replacement primitive expresses it.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enter_with_seven_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: DamageDealt has no self-only source filter; this is
                // approximated by "a creature you control".
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: gain_life_for_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enter_with_seven_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 7,
    }]
}

fn gain_life_for_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife {
        player: trig.controller,
        amount: n,
    }]
}
