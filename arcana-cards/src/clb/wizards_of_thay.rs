//! Wizards of Thay — `{3}{U}` 3/3 Human Wizard.
//! Myriad → GAP (not a keyword variant; token-copy-per-opponent attack mechanic).
//! "Instant and sorcery spells you cast cost {1} less to cast." → GAP (cost reduction).
//! "You may cast sorcery spells as though they had flash." → GAP (static permission).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wizards of Thay");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: Myriad — no keyword variant / token-copy-per-opponent mechanic.
    // GAP: "Instant and sorcery spells you cast cost {1} less" — cost reduction.
    // GAP: "You may cast sorcery spells as though they had flash" — static permission.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
