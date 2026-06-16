//! Pestilent Spirit — `{2}{B}` 3/2 black Spirit.
//! Menace, deathtouch.
//! GAP: "Instant and sorcery spells you control have deathtouch." — a static
//! continuous ability granting a keyword to your spells; not expressible as a
//! triggered/activated ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pestilent Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
