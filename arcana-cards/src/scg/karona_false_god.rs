//! Karona, False God — `{1}{W}{U}{B}{R}{G}` 5/5 Legendary Avatar with
//! Haste.
//! * "At the beginning of each player's upkeep, that player untaps Karona
//!   and gains control of it."
//! * "Whenever Karona attacks, creatures of the creature type of your
//!   choice get +3/+3 until end of turn."
//!
//! Haste is an engine keyword. Trigger 1 fires on each player's upkeep
//! ("that player" = the active player, who owns the upkeep) and untaps
//! Karona, then changes control to that player. Trigger 2's effect is
//! GAP'd — "creatures of the creature type of your choice" requires
//! choosing a creature type and pumping all creatures of it, which has no
//! expressible primitive.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karona, False God");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: untap_and_give_to_active,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_chosen_type,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn untap_and_give_to_active(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" = the active player, who owns the current upkeep.
    let them = state.active_player();
    vec![
        Effect::Untap { target: trig.source },
        Effect::ChangeControl {
            target: trig.source,
            new_controller: them,
        },
    ]
}

fn pump_chosen_type(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "creatures of the creature type of your choice get +3/+3" — no
    // primitive to choose a creature type and pump all creatures of that
    // type. Whole effect GAP'd.
    Vec::new()
}
