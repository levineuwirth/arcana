//! Magmatic Hellkite — `{2}{R}{R}` 4/5 Creature — Dragon with Flying.
//!
//! * Flying.
//! * When this creature enters, destroy target nonbasic land an opponent
//!   controls. Its controller searches their library for a basic land card,
//!   puts it onto the battlefield tapped, then shuffles.
//!
//! GAP (fidelity): the "with a stun counter on it" rider on the tutored land is
//! not expressible — `TutorToBattlefield` mints the land but its id isn't
//! available to place a counter on.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Magmatic Hellkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: destroy_land_and_ramp,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
                        .controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn destroy_land_and_ramp(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let owner = script::target_controller(state, *id, trig.controller);
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::TutorToBattlefield {
            player: owner,
            filter: ObjectFilter::permanent()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
            tapped: true,
        },
    ]
}
