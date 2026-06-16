//! Toxicrene — `{3}{G}` 2/4 Tyranid with Reach and Deathtouch.
//! "Hypertoxic Miasma — All lands have '{T}: Add one mana of any color' and
//! lose all other abilities." (static — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toxicrene");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: "Hypertoxic Miasma — All lands have '{T}: Add one mana of any color'
    // and lose all other abilities" — a board-wide ability-granting/stripping
    // static; not expressible on this shape.
    reg.register(CardDefinition::new(name, chars))
}
