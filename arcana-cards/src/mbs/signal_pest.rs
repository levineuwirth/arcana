//! Signal Pest — `{1}` 0/1 Artifact Creature — Pest.
//! Battle cry. The static "This creature can't be blocked except by creatures with
//! flying or reach" is a conditional (filtered) block-restriction static with no
//! expressible primitive (CantBeBlocked is unconditional) — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Signal Pest");
    let pest = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::BattleCry],
        ..Default::default()
    };

    // GAP: static "can't be blocked except by creatures with flying or reach"
    // (filtered block restriction not expressible).
    reg.register(CardDefinition::new(name, chars))
}
