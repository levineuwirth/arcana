//! Noble Hierarch — `{G}` 0/1 Human Druid with Exalted.
//!
//! Oracle:
//! * Exalted.
//! * `{T}: Add {G}, {W}, or {U}.` (A player-chosen color among three;
//!   no mana-choice primitive exists — GAP'd rather than emit a wrong
//!   fixed color.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Noble Hierarch");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Exalted],
        ..Default::default()
    };

    // GAP: "{T}: Add {G}, {W}, or {U}." — Effect::AddMana takes a fixed
    // mana vec; there is no player-chosen-color mana primitive, so this
    // tap-for-one-of-three-colors mana ability is omitted.

    reg.register(CardDefinition::new(name, chars))
}
