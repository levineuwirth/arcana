//! Dust Animus — `{1}{W}` 2/3 Creature — Spirit with Flying.
//!
//! Oracle text:
//! * Flying — base keyword.
//! * "If you control five or more untapped lands, this creature enters with
//!   two +1/+1 counters and a lifelink counter on it." — a conditional
//!   enters-with-counters REPLACEMENT effect. There is no enters-with /
//!   replacement primitive on the MultiAbilityCreature shape (no
//!   "this creature enters with N counters" Effect), so this is GAP'd.
//! * Plot {1}{W} — Plot is not an available `KeywordAbility`; GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dust Animus");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Plot is not in the usable keyword surface.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "If you control five or more untapped lands, this creature enters
    // with two +1/+1 counters and a lifelink counter on it." — conditional
    // enters-with-counters replacement effect; no such primitive available.
    reg.register(CardDefinition::new(name, chars))
}
