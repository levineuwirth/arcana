//! Mutagen Connoisseur — `{1}{G}{U}` 0/5 Vedalken Mutant with Flying and
//! Vigilance. "This creature gets +1/+0 for each transformed permanent
//! you control." (Static continuous self-pump — GAP'd below.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mutagen Connoisseur");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(mutant);

    // GAP: "This creature gets +1/+0 for each transformed permanent you
    // control" — a static continuous self-P/T modification keyed to a
    // dynamic board count; not expressible as a triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
