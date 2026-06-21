//! Maralen of the Mornsong — `{1}{B}{B}` Legendary 2/3 Elf Wizard.
//! Players can't draw cards.
//! At the beginning of each player's draw step, that player loses 3 life,
//! searches their library for a card, puts it into their hand, then
//! shuffles.
//!
//! GAP: the static "Players can't draw cards." is a draw-replacement /
//! prohibition with no demonstrated Effect / continuous-static primitive,
//! so it is omitted. The draw-step trigger (life loss + tutor to hand) is
//! implemented for the active player whose draw step it is.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maralen of the Mornsong");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    // GAP: static "Players can't draw cards." — no draw-prohibition primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Draw,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: draw_step_lose_and_tutor,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "that player loses 3 life, searches their library for a card, puts it
/// into their hand, then shuffles." — the draw step belongs to the active
/// player, and this trigger resolves during that same draw step.
fn draw_step_lose_and_tutor(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let them = state.active_player();
    vec![
        Effect::LoseLife {
            player: them,
            amount: 3,
        },
        Effect::TutorToHand {
            player: them,
            filter: ObjectFilter::default(),
            reveal: false,
        },
    ]
}
