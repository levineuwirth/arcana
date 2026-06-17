//! Ian Malcolm, Chaotician — `{1}{U}{R}` 2/2 Legendary Human Scientist.
//! "Whenever a player draws their second card each turn, that player exiles
//! the top card of their library."
//! "During each player's turn, that player may cast a spell from among the
//! cards they don't own exiled with Ian Malcolm, and mana of any type can be
//! spent to cast it."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ian Malcolm, Chaotician");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP (static): "During each player's turn, that player may cast a spell
    // from among the cards they don't own exiled with Ian Malcolm, and mana of
    // any type can be spent to cast it." — a continuous play-permission with no
    // triggered/activated/keyword representation. Omitted.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: second_draw_exile,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn second_draw_exile(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "draws their SECOND card each turn" cannot be gated (no per-drawer
    // second-draw intervening-if predicate), and "that player exiles the top of
    // THEIR library" cannot reach the drawing player (no drawing-player accessor
    // on PendingTrigger). Whole effect omitted to avoid firing on every draw
    // against the wrong player.
    Vec::new()
}
