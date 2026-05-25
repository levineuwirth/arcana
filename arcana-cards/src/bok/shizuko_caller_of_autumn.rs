//! Shizuko, Caller of Autumn — `{1}{G}{G}` 2/3 Legendary Snake Shaman.
//! "At the beginning of each player's upkeep, that player adds
//! {G}{G}{G}. Until end of turn, they don't lose this mana as steps
//! and phases end."
//!
//! GAP: "don't lose this mana as steps and phases end" — no catalog
//! variant for persistent/floating mana that doesn't empty between steps.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shizuko, Caller of Autumn");
    let snake = reg.interner_mut().intern("Snake");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
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
                effect: on_upkeep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_upkeep(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "that player" — the active player (whose upkeep it is).
    // The trigger fires once per upkeep; trig.controller gives the
    // ability's controller; the active player whose upkeep it is
    // should be trig.defending_player or equivalent, but no accessor
    // exposes active player directly — use trig.controller as proxy.
    // GAP: "don't lose this mana as steps and phases end" — no catalog
    // variant for persistent mana.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, trig.source),
            ManaUnit::plain(ManaColor::Green, trig.source),
            ManaUnit::plain(ManaColor::Green, trig.source),
        ],
    }]
}
