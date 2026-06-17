//! Galea, Kindler of Hope — `{1}{G}{W}{U}` 4/4 Legendary Elf Knight (G/W/U).
//! Vigilance.
//! You may look at the top card of your library any time.
//! You may cast Aura and Equipment spells from the top of your library.
//! When you cast an Equipment spell this way, it gains "When this
//! Equipment enters, attach it to target creature you control."
//!
//! Vigilance is emitted. The top-of-library look and the cast-from-top
//! play-permission statics (plus the granted equipment attach rider) are
//! continuous play-permission effects with no expressible primitive —
//! GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galea, Kindler of Hope");
    let elf = reg.interner_mut().intern("Elf");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "You may look at the top card of your library any time." Static.
    // GAP: "You may cast Aura and Equipment spells from the top of your
    //      library..." Static play-permission + granted attach rider — no
    //      expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
