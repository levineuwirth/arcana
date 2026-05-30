//! Soundwave, Sonic Spy // Soundwave, Superior Captain
//! `{1}{W}{U}{B}` Legendary Artifact Creature — Robot 5/4 (front)
//! Legendary Artifact (back)
//!
//! Keywords: More Than Meets the Eye (GAP: not in engine keyword surface),
//!           Convert (GAP: not in engine keyword surface).
//!
//! Front face ability:
//! Whenever one or more creature tokens you control deal combat damage to a player,
//! exile target instant or sorcery card with mana value equal to the damage dealt
//! from their graveyard. Copy it. You may cast the copy without paying its mana cost.
//! If you do, convert Soundwave.
//! (GAP: "exile card with mana value equal to damage dealt, copy it, cast for free" —
//!  damage-value-gated graveyard exile + copy + free-cast not expressible; emit Vec::new().)
//!
//! Back face abilities:
//! Whenever you cast a spell with an odd mana value, convert Soundwave.
//! If you do, create Ravage, a legendary 3/3 black Robot artifact creature token
//! with menace and deathtouch.
//! Whenever you cast a spell with an even mana value, convert Soundwave.
//! If you do, create Laserbeak, a legendary 2/2 blue Robot artifact creature token
//! with flying and hexproof.
//! (GAP: back-face-only triggered abilities not modeled in current engine;
//!  odd/even mana value spell-cast trigger condition not in TriggerCondition set.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soundwave, Sonic Spy");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: More Than Meets the Eye and Convert are not in engine keyword surface
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Soundwave, Superior Captain — Legendary Artifact (no P/T)
    let back_name = reg.interner_mut().intern("Soundwave, Superior Captain");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: front-face trigger "whenever creature tokens deal combat damage to a player,
    //   exile instant/sorcery of equal mana value, copy it, cast for free, convert" —
    //   combat-damage-amount-gated graveyard exile + copy + free-cast not expressible.
    // GAP: back-face triggered abilities (odd/even mana value spell-cast → convert +
    //   create named legendary token) not modeled; back-face-only triggers not supported
    //   and odd/even mana value TriggerCondition does not exist.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
