//! Glitch Ghost Surveyor — `{2}{U}` 2/2 Spirit Scout.
//!
//! "Flying.
//!  Start your engines! (If you have no speed, it starts at 1...)
//!  Max speed — {3}, Exile this card from your graveyard: Draw a card."
//!
//! Flying is a base keyword. "Start your engines!"/speed is not modeled
//! (no speed mechanic) and is GAP'd. The Max speed ability is wired as a
//! graveyard-activated ability ({3}, exile this card from your graveyard:
//! draw a card); its "max speed" precondition (speed == 4) has no
//! `activation_condition` predicate available, so the gate is GAP'd — the
//! ability is otherwise faithful.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glitch Ghost Surveyor");
    let spirit = reg.interner_mut().intern("Spirit");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Start your engines!" / speed mechanic — not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Max speed — {3}, Exile this card from your graveyard: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    exile_self: true,
                    // GAP: "Max speed" precondition (speed == 4) has no
                    // activation_condition predicate available.
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            }),
    )
}

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
