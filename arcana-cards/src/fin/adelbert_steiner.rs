//! Adelbert Steiner — `{1}{W}` Legendary 2/1 Creature — Human Knight.
//! "Lifelink"
//! "Adelbert Steiner gets +1/+1 for each Equipment you control."
//!
//! Decomposition:
//! - Keyword line: Lifelink.
//! - GAP: "gets +1/+1 for each Equipment you control" is a static dynamic-P/T
//!   continuous ability — no expressible Effect.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adelbert Steiner");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
