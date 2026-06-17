//! A-Cobbled Lancer — `{U}` 3/3 Zombie Horse.
//! "As an additional cost to cast this spell, exile a creature card from
//! your graveyard." (additional cast cost — GAP'd, no field for it)
//! "{1}{U}, Exile Cobbled Lancer from your graveyard: Draw a card."
//!   (graveyard-activated ability — expressible)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Cobbled Lancer");
    let zombie = reg.interner_mut().intern("Zombie");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horse);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "As an additional cost to cast this spell, exile a creature
        // card from your graveyard" — no additional-cast-cost field.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{U}, Exile Cobbled Lancer from your graveyard: Draw a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_a_card,
        }),
    )
}

fn draw_a_card(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
