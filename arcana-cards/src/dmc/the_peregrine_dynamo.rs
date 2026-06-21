//! The Peregrine Dynamo — `{3}` 1/5 Legendary Artifact Creature — Construct, Haste.
//! `{1}, {T}: Copy target activated or triggered ability you control from another
//! legendary source that's not a commander…` → GAP (no copy-ability Effect;
//! CopySpell only copies spells, and the legendary/commander source restriction
//! is not expressible).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Peregrine Dynamo");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    // GAP: "{1}, {T}: Copy target activated or triggered ability you control …" —
    //      no copy-ability Effect; CopySpell targets spells only.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
