//! Archon of Emeria — `{2}{W}` 2/3 Archon with Flying.
//! "Each player can't cast more than one spell each turn.
//!  Nonbasic lands your opponents control enter tapped."
//!
//! Both lines are pure continuous STATIC abilities (a spell-count
//! restriction and an enters-tapped replacement on opponents'
//! nonbasic lands) — neither is a triggered/activated ability and no
//! demonstrated primitive expresses them. Both GAP'd; Flying emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archon of Emeria");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Each player can't cast more than one spell each turn" — global spell-count restriction; no demonstrated primitive.
    // GAP: static "Nonbasic lands your opponents control enter tapped" — enters-tapped replacement on opponents' permanents; no demonstrated primitive.
    reg.register(CardDefinition::new(name, chars))
}
