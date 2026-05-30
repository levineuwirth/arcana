//! Chaplain of Alms // Chapel Shieldgeist — `{W}` Creature — Human Cleric 1/1 (front) /
//! Creature — Spirit Cleric (back). Transform (Disturb).
//!
//! Front face:
//!   First strike
//!   Ward {1}
//!   Disturb {3}{W} (cast from graveyard transformed; not modeled — engine debt)
//!
//! Back face (Chapel Shieldgeist):
//!   Flying, first strike
//!   Each creature you control has ward {1}. (GAP: static continuous grant not expressible)
//!   If Chapel Shieldgeist would be put into a graveyard from anywhere, exile it instead.
//!     (GAP: replacement effect "exile instead of graveyard" not expressible)
//!
//! GAP: Disturb (cast from graveyard transformed) — engine debt; not modeled.
//! GAP: back-face-only triggered/static abilities not auto-installed on transform.
//! GAP: "each creature you control has ward {1}" — static continuous grant not expressible.
//! GAP: "if this would be put into a graveyard, exile it instead" — replacement effect not expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaplain of Alms");
    let human_sub = reg.interner_mut().intern("Human");
    let cleric_sub = reg.interner_mut().intern("Cleric");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid ward cost")),
        ],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Chapel Shieldgeist");
    let back_spirit_sub = reg.interner_mut().intern("Spirit");
    let back_cleric_sub = reg.interner_mut().intern("Cleric");

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_spirit_sub);
    back_subtypes.0.insert(back_cleric_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: Disturb — cast from graveyard transformed — engine debt; not modeled.
        // GAP: back-face "each creature you control has ward {1}" — static continuous grant not expressible.
        // GAP: back-face "if this would be put into a graveyard, exile it instead" — replacement effect not expressible.
        // GAP: back-face-only triggered ability not modeled.
    )
}
