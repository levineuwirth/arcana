//! Thran Tome — `{4}` artifact — Book.
//! "{5}, {T}: Reveal the top three cards of your library. Target
//! opponent chooses one of those cards. Put that card into your
//! graveyard, then draw two cards."
//!
//! Approximated as: mill one card (the opponent's pick is modeled as
//! the top card — a documented fidelity gap; no opponent-chooses
//! primitive exists), then draw two.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thran Tome");
    let book = reg.interner_mut().intern("Book");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(book);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}, {T}: Reveal the top three cards of your library. Target opponent chooses one of those cards. Put that card into your graveyard, then draw two cards.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: reveal_pick_draw,
        }),
    )
}

fn reveal_pick_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Target opponent chooses one of those cards" — no
    // opponent-chooses-from-revealed primitive; approximated by milling the
    // top card (one of the three) to the graveyard, then drawing two.
    vec![
        Effect::Mill { player: ctx.controller, count: 1 },
        Effect::DrawCards { player: ctx.controller, count: 2 },
    ]
}
