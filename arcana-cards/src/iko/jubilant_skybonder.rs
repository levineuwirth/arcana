//! Jubilant Skybonder — `{1}{W/U}{W/U}` Creature — Human Wizard, 2/2.
//!
//! Oracle:
//! * Flying.
//! * Creatures you control with flying have "Spells your opponents cast that
//!   target this creature cost {2} more to cast."
//!
//! The second line is a pure static continuous ability (grants a cost-increase
//! static to a set of creatures) with no trigger word and no activation cost.
//! It is not expressible as a triggered/activated ability and is GAP'd; only
//! the Flying keyword is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jubilant Skybonder");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W/U}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Creatures you control with flying have 'Spells your opponents
    // cast that target this creature cost {2} more to cast.'" — a granted
    // cost-increase static continuous ability, not a triggered/activated ability;
    // not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
