//! Boon Satyr — `{1}{G}{G}` 4/2 Enchantment Creature — Satyr, with Flash.
//! "Flash. Bestow {3}{G}{G}. Enchanted creature gets +4/+2."
//!
//! Flash is a base characteristic. Bestow is not a KeywordAbility variant and
//! the bestow alternate-cost Aura mode is not expressible, so the bestow
//! mechanic and its "enchanted creature gets +4/+2" static buff are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boon Satyr");
    let satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: "Bestow {3}{G}{G}" — Bestow is not a KeywordAbility variant; the
    // bestow alternate-cost Aura mode and its "enchanted creature gets +4/+2"
    // static are not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
