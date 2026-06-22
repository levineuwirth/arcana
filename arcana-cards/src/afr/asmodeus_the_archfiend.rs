//! Asmodeus the Archfiend — `{4}{B}{B}` 6/6 Legendary Creature — Devil God.
//!
//! Oracle:
//! * Binding Contract — If you would draw a card, exile the top card of your
//!   library face down instead.
//! * {B}{B}{B}: Draw seven cards.
//! * {B}: Return all cards exiled with Asmodeus to their owner's hand and you
//!   lose that much life.
//!
//! Decomposition: the {B}{B}{B} activated ability (draw seven cards) is
//! expressible.
//!
//! GAP: "Binding Contract — if you would draw a card, exile the top card of
//!      your library face down instead" is a draw-replacement static; not a
//!      triggered/activated ability; omitted.
//! GAP: "{B}: Return all cards exiled with Asmodeus to their owner's hand and
//!      you lose that much life" references the set of cards exiled by this
//!      specific source and a dynamic life loss tied to that count — no
//!      primitive tracks per-source exile sets; omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Asmodeus the Archfiend");
    let devil = reg.interner_mut().intern("Devil");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}{B}{B}: Draw seven cards.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}{B}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_seven,
        }),
    )
}

fn draw_seven(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 7,
    }]
}
