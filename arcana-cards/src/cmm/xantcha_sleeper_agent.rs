//! Xantcha, Sleeper Agent — `{1}{B}{R}` 5/5 Legendary Phyrexian Minion.
//! Enters under an opponent's control; attacks each combat if able and can't
//! attack its owner; {3}: Xantcha's controller loses 2 life and you draw a
//! card. Any player may activate this ability.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xantcha, Sleeper Agent");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(minion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "enters under the control of an opponent of your choice" — no ETB
    // control-assignment effect/replacement.
    // GAP: static — "attacks each combat if able and can't attack its owner or
    // planeswalkers its owner controls" — no attack-requirement/restriction
    // static primitive.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP (partial): "Xantcha's controller loses 2 life" — no
            // source-controller accessor in the activated effect (ctx.controller
            // is the activator, not necessarily Xantcha's controller). The draw
            // is the activator's; the controller life-loss is omitted.
            // GAP: "Any player may activate this ability" — no activator-scope
            // override field.
            text: "{3}: Xantcha's controller loses 2 life and you draw a card. Any player may activate this ability.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_card,
        }),
    )
}

fn draw_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
