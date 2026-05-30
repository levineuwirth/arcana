//! Khenra Spellspear // Gitaxian Spellstalker — {1}{R} Creature — Jackal Warrior // Phyrexian Jackal (2/2)
//! Front: Trample, Prowess (GAP: Prowess not in keyword surface).
//!   {3}{U/P}: Transform this creature. Activate only as a sorcery.
//!   (GAP: hybrid/phyrexian mana {U/P} cost not expressible in ManaCost::parse.)
//! Back: Trample, Ward {2}, Prowess, Prowess (each instance triggers separately).
//!   (GAP: Prowess not in keyword surface; duplicate prowess not modeled.)
//! GAP: Activated transform ability cost {3}{U/P} — phyrexian hybrid mana not supported.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Khenra Spellspear");
    let jackal_sub = reg.interner_mut().intern("Jackal");
    let warrior_sub = reg.interner_mut().intern("Warrior");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(jackal_sub);
    front_subtypes.0.insert(warrior_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Trample,
            // GAP: Prowess not in engine keyword surface.
        ],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Gitaxian Spellstalker");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let jackal_sub2 = reg.interner_mut().intern("Jackal");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(jackal_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![
                KeywordAbility::Trample,
                KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
                // GAP: Prowess (x2) not in engine keyword surface.
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: Activated transform ability {3}{U/P} — phyrexian hybrid mana not expressible.
            // Transform trigger approximated via SpellCast trigger is not correct semantics.
            // Omitting the activated ability.
    )
}
