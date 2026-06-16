//! Lona, Tracker of the Known — `{1}{G}` 1/1 Legendary Elf Ranger.
//! "Spells you cast that have been printed in at least five different English
//! language Magic releases cost {W}, {U}, {B}, {R}, or {G} less to cast."
//! "Lona gets +1/+1 for each nonland permanent you control that has been printed
//! in at least five different English language Magic releases."
//!
//! Both abilities key off card print-history (number of English releases) — a
//! metadata metric with no game-state representation and no expressible
//! primitive in this class. Both GAP'd; bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lona, Tracker of the Known");
    let elf = reg.interner_mut().intern("Elf");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static cost-reduction keyed on a card's print history (>=5 English
    // releases) — no game-state metric or cost-reduction primitive here.
    // GAP: static dynamic +1/+1 per nonland permanent with the same print-history
    // metric — unrepresentable.
    reg.register(CardDefinition::new(name, chars))
}
