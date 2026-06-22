//! The Mox Painter — `{1}{G}{U}` 1/4 Legendary Creature — Human Artist.
//!
//! `{1}, {T}:` Choose a tournament-legal Mox you don't control at random.
//!   Create a token that's a copy of it. Then if you control all ten,
//!   create a Mox Lotus token.
//! `{W}{U}{B}{R}{G}:` Untap The Mox Painter.
//!
//! The first ability requires choosing one of the ten Moxen at random and
//! minting a token copy of a card the engine has no registry-by-name
//! random-pick primitive for (no "choose at random from named cards"
//! Effect), so it is GAP'd. The untap activation is fully expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Mox Painter");
    let human = reg.interner_mut().intern("Human");
    let artist = reg.interner_mut().intern("Artist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artist);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Choose a tournament-legal Mox you don't control at random. Create a token that's a copy of it. Then if you control all ten, create a Mox Lotus token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: choose_random_mox,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}{B}{R}{G}: Untap The Mox Painter.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_self,
            }),
    )
}

fn choose_random_mox(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose a tournament-legal Mox you don't control at random; create a token
    // that's a copy of it; then if you control all ten, create a Mox Lotus token" — there
    // is no registry-by-name random-pick primitive (the Moxen aren't on the battlefield to
    // CopyPermanent, and there is no Conjure/name-a-card-and-mint effect).
    Vec::new()
}

fn untap_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
