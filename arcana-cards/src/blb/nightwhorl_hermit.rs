//! Nightwhorl Hermit — `{2}{U}` 1/4 Rat Rogue with Vigilance.
//! "Threshold — As long as there are seven or more cards in your
//! graveyard, this creature gets +1/+0 and can't be blocked."
//!
//! Vigilance is a base keyword (Threshold is the ability-word label,
//! not a KeywordAbility). The Threshold clause is a conditional STATIC
//! continuous self-buff — neither triggered nor activated — and there
//! is no Effect to express a graveyard-gated static, so it is GAP'd.

// GAP (static): "Threshold — as long as there are seven or more cards in your
// graveyard, this creature gets +1/+0 and can't be blocked." A conditional
// continuous self-buff has no triggered/activated decomposition here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nightwhorl Hermit");
    let rat = reg.interner_mut().intern("Rat");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
