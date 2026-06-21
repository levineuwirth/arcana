//! Goblin Surveyor — `{2}{R}` 3/2 Goblin Scout.
//! Trample.
//! Start your engines! / Max speed (GAP'd — the speed mechanic has no
//! usable keyword or primitive).
//! Max speed — {3}, Exile this card from your graveyard: Draw a card.
//! (modeled as a graveyard-activated {3} + exile-self ability; the
//! "max speed" precondition is a fidelity gap — no speed state hook.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Surveyor");
    let goblin = reg.interner_mut().intern("Goblin");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        // GAP: "Start your engines!" / speed mechanic — no usable keyword.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Max speed — {3}, Exile this card from your graveyard: Draw a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                exile_self: true,
                // GAP: "Max speed" precondition (speed == 4) has no
                // expressible activation_condition hook.
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
