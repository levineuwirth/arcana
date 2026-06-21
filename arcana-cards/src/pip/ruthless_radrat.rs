//! Ruthless Radrat — `{1}{B}{B}` 2/2 Rat Mutant.
//!
//! Oracle:
//! * "Squad—Exile four cards from your graveyard." — Squad is not a usable
//!   KeywordAbility variant, and the "pay the squad cost any number of times,
//!   then create that many token copies on ETB" additional-cost machinery is
//!   not expressible. GAP'd.
//! * Menace.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruthless Radrat");
    let rat = reg.interner_mut().intern("Rat");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Squad—Exile four cards from your graveyard." (additional-cost-any-
    // number-of-times → ETB token copies) is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
