//! Vantress Gargoyle — `{1}{U}` 5/4 blue Artifact Creature — Gargoyle.
//!
//! Oracle:
//! * Flying → `keywords`.
//! * "This creature can't attack unless defending player has seven or
//!   more cards in their graveyard." — a static attack restriction with
//!   no trigger word / cost; not expressible. GAP'd.
//! * "This creature can't block unless you have four or more cards in
//!   hand." — a static block restriction; not expressible. GAP'd.
//! * "{T}: Each player mills a card." — a tap activated ability milling
//!   every player. We enumerate all players and emit one `Mill` each
//!   inside a `Sequence`.
//!
//! (Scryfall lists "Mill" as a keyword only because the activated ability
//! mills; there is no `KeywordAbility::Mill`, so it's expressed through
//! the activated ability below, not the keyword line.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vantress Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "can't attack unless defending player has seven or more
    // cards in their graveyard" — attack restriction, no API surface.
    // GAP: static "can't block unless you have four or more cards in hand"
    // — block restriction, no API surface.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Each player mills a card.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: each_player_mills,
        }),
    )
}

fn each_player_mills(
    state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 1 })
        .collect();
    vec![Effect::Sequence(effects)]
}
