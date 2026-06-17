//! Fathom Feeder — `{U}{B}` 1/1 Eldrazi Drone. Devoid (colorless),
//! Deathtouch, Ingest (combat damage to a player → that player exiles the
//! top card of their library — no exile-top-of-library effect available →
//! GAP). `{3}{U}{B}: Draw a card. Each opponent exiles the top card of their
//! library` (the exile half is GAP'd, same reason).

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
    let name = reg.interner_mut().intern("Fathom Feeder");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: Ingest ("whenever this creature deals combat damage to a player,
    // that player exiles the top card of their library") — no
    // exile-top-of-library effect in the available surface.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}{B}: Draw a card. Each opponent exiles the top card of their library.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}{B}").expect("valid cost"),
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
    // GAP: "Each opponent exiles the top card of their library" — no
    // exile-top-of-library effect available; only the draw is emitted.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
