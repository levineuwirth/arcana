//! Emberwilde Djinn — `{2}{R}{R}` 5/4 Djinn with Flying.
//! "At the beginning of each player's upkeep, that player may pay {R}{R} or
//! 2 life. If the player does, they gain control of this creature."

use arcana_core::actions::OptionalPaymentKind;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emberwilde Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
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
                effect: upkeep_ransom,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_ransom(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The upkeep belongs to the active player; that player is the one who may
    // pay to gain control.
    let them = state.active_player();
    // GAP: the "or 2 life" alternative cost can't be combined with the {R}{R}
    // mana option in a single OptionalPayment; we wire the {R}{R} branch.
    vec![Effect::OptionalPayment {
        chooser: them,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}{R}").expect("valid cost")),
        then: Box::new(Effect::ChangeControl {
            target: trig.source,
            new_controller: them,
        }),
        else_effect: None,
    }]
}
