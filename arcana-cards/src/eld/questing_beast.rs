//! Questing Beast — `{2}{G}{G}` 4/4 Legendary Beast.
//!
//! Vigilance, deathtouch, haste.
//! Questing Beast can't be blocked by creatures with power 2 or less. (static — GAP)
//! Combat damage that would be dealt by creatures you control can't be
//! prevented. (static — GAP)
//! Whenever Questing Beast deals combat damage to an opponent, it deals
//! that much damage to target planeswalker that player controls.
//!
//! Three keywords (base characteristics) plus a combat-damage trigger.
//! The two static lines (a block restriction and an unpreventable-
//! damage replacement) have no trigger word or activation cost and are
//! not expressible with the triggered/activated primitives — GAP'd.

use arcana_core::effects::Effect;
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
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Questing Beast");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // Source filter restricting the combat-damage trigger to this
    // creature alone ("Questing Beast deals combat damage").
    let self_name = reg.interner().lookup("Questing Beast");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Deathtouch,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    // GAP: static "can't be blocked by creatures with power 2 or less"
    // — a conditional block restriction (pure static, no trigger/cost).
    // GAP: static "combat damage dealt by creatures you control can't be
    // prevented" — a board-wide replacement modifier (pure static).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter {
                    name: self_name,
                    ..ObjectFilter::default()
                },
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: damage_planeswalker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types(TypeLine::PLANESWALKER.into())
                        .controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

/// Deal damage equal to the combat damage just dealt to target
/// planeswalker the damaged player controls.
fn damage_planeswalker(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
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
