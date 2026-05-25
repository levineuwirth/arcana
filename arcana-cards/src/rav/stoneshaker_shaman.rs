//! Stoneshaker Shaman — `{2}{R}` 1/1 red Creature — Human Shaman.
//! "At the beginning of each player's end step, that player sacrifices an
//! untapped land of their choice."
//! GAP: "that player sacrifices an untapped land" — Sacrifice effect uses
//! ObjectFilter but no "that player" accessor (the triggering player); also
//! untapped_only land filter. Using StepBegins/End/Any and best-effort.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stoneshaker Shaman");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: on_end_step_sacrifice_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_end_step_sacrifice_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player" (the player whose end step it is) not accessible via
    // trig; using trig.controller as approximation. Also "untapped land" filter
    // for Sacrifice uses untapped_only() but no land-type filter.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()).untapped_only(),
        count: 1,
    }]
}
