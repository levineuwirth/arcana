//! Chandra's Incinerator — `{5}{R}` 6/6 Creature — Elemental.
//!
//! * "This spell costs {X} less to cast, where X is the total noncombat
//!   damage dealt to your opponents this turn." — a cost-reduction static,
//!   not a triggered/activated ability.
//! * Trample.
//! * "Whenever a source you control deals noncombat damage to an opponent,
//!   this creature deals that much damage to target creature or planeswalker
//!   that player controls."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra's Incinerator");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: cost-reduction static "costs {X} less = noncombat damage to your
    // opponents this turn" — a continuous cost modifier, not an ability def.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "noncombat" damage cannot be filtered (combat_only is a
                // bool that only isolates combat damage); this fires on any
                // damage a permanent you control deals to an opponent.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: mirror_damage,
                trigger_zones: vec![Zone::Battlefield],
                target_requirements: vec![TargetRequirement {
                    // GAP: "that player controls" cannot be expressed in the
                    // target filter (no link to the damaged player); any
                    // creature or planeswalker is targetable.
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                frequency: TriggerFrequency::EachTime,
            }),
    )
}

fn mirror_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let amount = trig.damage_amount().unwrap_or(0);
    if amount == 0 {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
