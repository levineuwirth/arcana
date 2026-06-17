//! Necrogoyf — `{3}{B}{B}` */4 black Lhurgoyf.
//! Power equals the number of creature cards in all graveyards (CDA).
//! At the beginning of each player's upkeep, that player discards a card.
//! Madness {1}{B}{B} — not in the usable keyword surface; GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Necrogoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    // GAP: power is a characteristic-defining ability (* = number of creature
    // cards in all graveyards); no CDA primitive in the usable surface.
    // GAP: keyword Madness {1}{B}{B} — not in the usable keyword surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: each_player_upkeep_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_player_upkeep_discard(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let them = state.active_player();
    vec![Effect::Discard {
        player: them,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
