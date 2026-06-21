//! Gargantuan Leech — `{7}{B}` 5/5 Leech with Lifelink.
//! "This spell costs {1} less to cast for each Cave you control and each
//!  Cave card in your graveyard.
//!  Lifelink"

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gargantuan Leech");
    let leech = reg.interner_mut().intern("Leech");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leech);

    // GAP: "costs {1} less for each Cave you control and Cave card in
    // your graveyard" — no cost-reduction static expressible in this
    // creature shape.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
