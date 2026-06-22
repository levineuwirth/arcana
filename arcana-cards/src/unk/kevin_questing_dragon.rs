//! Kevin, Questing Dragon — `{4}{R}{R}{R}{R}` 8/8 Legendary Dragon.
//! "Kevin can't be countered" — GAP (no can't-be-countered effect).
//! Devour 2, Flying, Mountainwalk, Rampage 2, Bushido 2, Trample
//!  (trample "over planeswalkers" modeled as plain Trample).
//! "Damage can't be prevented." / "Players can't gain life." — GAP (statics).
//! "Whenever Kevin deals combat damage to a player, gain control of target
//!  land that player controls. Untap it."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kevin, Questing Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let mountain = reg.interner_mut().intern("Mountain");

    // GAP (statics): "Kevin can't be countered", "Damage can't be prevented",
    // "Players can't gain life" — none expressible as base characteristics.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Landwalk(mountain),
            KeywordAbility::Rampage(2),
            KeywordAbility::Bushido(2),
            KeywordAbility::Devour(2),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: steal_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn steal_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // FIDELITY GAP: target restricted to a land controlled by an opponent
    // rather than specifically the damaged player.
    vec![
        Effect::ChangeControl {
            target: *id,
            new_controller: trig.controller,
        },
        Effect::Untap { target: *id },
    ]
}
