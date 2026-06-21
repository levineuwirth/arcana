//! Street Wraith — `{3}{B}{B}` 3/4 Wraith. Swampwalk.
//! "Cycling—Pay 2 life. (Pay 2 life, Discard this card: Draw a card.)"
//! The Cycling keyword variant only carries a MANA cost; this card's
//! cycling cost is a life payment, so it is modeled directly as a
//! hand-activated ability (Pay 2 life, discard self: draw a card).

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
    let name = reg.interner_mut().intern("Street Wraith");
    let wraith = reg.interner_mut().intern("Wraith");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wraith);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Cycling—Pay 2 life. (Pay 2 life, Discard this card: Draw a card.)".into(),
            cost: ActivationCost {
                life: 2,
                discard_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Hand,
            is_instant_speed: true,
            face_gate: None,
            effect: cycle_draw,
        }),
    )
}

fn cycle_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
