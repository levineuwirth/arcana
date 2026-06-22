//! Sunblade Samurai — `{4}{W}` 4/4 Enchantment Creature — Human Samurai.
//!
//! * Vigilance.
//! * `Channel — {2}, Discard this card: Search your library for a basic Plains
//!   card, reveal it, put it into your hand, then shuffle. You gain 2 life.`
//!   Activated from hand (`{2}` + discard this card), tutoring a basic Plains
//!   to hand and gaining 2 life.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunblade Samurai");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    // Pre-intern the Plains subtype so the resolver's tutor filter resolves it.
    let _plains = reg.interner_mut().intern("Plains");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Channel — {2}, Discard this card: Search your library for a basic Plains card, reveal it, put it into your hand, then shuffle. You gain 2 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: channel_plains,
            }),
    )
}

fn channel_plains(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plains = script::subtype_filter(reg, "Plains")
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![
        Effect::TutorToHand {
            player: ctx.controller,
            filter: plains,
            reveal: true,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 2,
        },
    ]
}
