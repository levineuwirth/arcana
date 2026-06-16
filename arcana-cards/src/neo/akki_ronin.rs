//! Akki Ronin — `{1}{R}` 1/3 red Goblin Samurai.
//! "Whenever a Samurai or Warrior you control attacks alone, you may discard a card.
//! If you do, draw a card."
//! "Attacks alone" via TriggerCondition::AttacksAlone over Samurai-or-Warrior you control.
//! GAP: "you may discard" optionality not modeled; discard+draw is mandatory.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akki Ronin");
    let goblin = reg.interner_mut().intern("Goblin");
    let samurai = reg.interner_mut().intern("Samurai");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(samurai);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::AttacksAlone {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_subtypes_any(vec![samurai, warrior]),
                },
                intervening_if: None,
                effect: on_attacks_alone,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks_alone(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard" optionality not modeled; discard+draw is mandatory.
    vec![
        Effect::Discard { player: trig.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}
