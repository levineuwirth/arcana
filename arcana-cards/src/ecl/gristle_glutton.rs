//! Gristle Glutton — `{1}{R}` 1/3 red Goblin Scout.
//! "{T}, Blight 1: Discard a card. If you do, draw a card."
//! GAP: "Blight 1" cost (put a -1/-1 counter on a creature you control) not
//! in ActivationCost; modeled as tap-only. The discard-then-draw is also
//! conditional ("If you do") — OptionalPayment discard not supported; modeled
//! as unconditional discard+draw.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gristle Glutton");
    let goblin = reg.interner_mut().intern("Goblin");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(scout);
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Blight 1: Discard a card. If you do, draw a card.".into(),
                // GAP: Blight 1 cost (put -1/-1 on a creature you control) not
                // in ActivationCost; tap-only used as placeholder
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: discard_draw,
            }),
    )
}

fn discard_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Discard { player: ctx.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}
