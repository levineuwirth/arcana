//! Bastion Inventor — `{5}{U}` 4/4 Vedalken Artificer with Hexproof.
//!
//! Oracle:
//! * Improvise (your artifacts can help cast this spell). (GAP — Improvise is
//!   a cost-reduction casting mechanic, not in the expressible keyword surface.)
//! * Hexproof (keyword).
//!
//! Only Hexproof is expressible. Improvise has no `KeywordAbility` variant and
//! no cost-reduction effect representation, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bastion Inventor");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(artificer);

    // GAP: "Improvise" — a casting cost-reduction mechanic with no expressible
    // KeywordAbility variant or cost-reduction effect.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
