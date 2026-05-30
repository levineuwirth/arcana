//! Mila, Crafty Companion // Lukka, Wayward Bonder — MDFC.
//! Front: `{1}{W}{W}` Legendary Creature — Fox 2/3.
//!   Whenever an opponent attacks one or more planeswalkers you control, put a loyalty
//!   counter on each planeswalker you control.
//!   Whenever a permanent you control becomes the target of a spell or ability an opponent
//!   controls, you may draw a card.
//! Back: `{3}{R}{G}` Legendary Planeswalker — Lukka (starting loyalty 4).
//!   +1: You may discard a card. If you do, draw a card.
//!         If a creature card was discarded this way, draw two cards instead.
//!   −2: Return target creature card from your graveyard to the battlefield. It gains
//!         haste. Exile it at the beginning of your next upkeep.
//!   −7: (Emblem — not expressible.)
//!
//! # GAPs
//! - Front trigger "whenever an opponent attacks one or more planeswalkers you control":
//!   no AttacksTargetPlaneswalker trigger condition — GAP.
//! - Front trigger "whenever a permanent you control becomes the target of a spell or
//!   ability an opponent controls": no BecomesTarget trigger condition — GAP.
//! - Back +1 "if a creature card was discarded, draw two instead": the conditional
//!   draw-2-vs-draw-1 branch is not expressible — modeled as simple discard-then-draw-1.
//! - Back −7 emblem "Whenever a creature you control enters, it deals damage equal to its
//!   power to any target": Emblem not modeled — GAP.
//! - Back-face-only triggered abilities not auto-installed on transform.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    // Front face
    let name = reg.interner_mut().intern("Mila, Crafty Companion");
    let fox_sub = reg.interner_mut().intern("Fox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "whenever an opponent attacks one or more planeswalkers you control" trigger
        // GAP: "whenever a permanent you control becomes the target of a spell or ability
        //       an opponent controls, you may draw a card" trigger
        ..Default::default()
    };

    // Back face — Lukka, Wayward Bonder (Legendary Planeswalker, starting loyalty 4)
    let back_name = reg.interner_mut().intern("Lukka, Wayward Bonder");
    let lukka_sub = reg.interner_mut().intern("Lukka");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(lukka_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid back cost")),
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(4),
            ..Default::default()
        },
        // Back-face spell ability wired as the MDFC back; planeswalker abilities are
        // expressed via activated abilities (not yet back-face-gated). GAP for back-face
        // planeswalker loyalty abilities not fully expressible here.
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back),
    )
}
