//! Jaheira, Friend of the Forest — `{2}{G}` 2/3 Legendary Human Elf Druid.
//!
//! * "Tokens you control have '{T}: Add {G}.'" — GAP: a static that GRANTS
//!   an activated mana ability to a filtered set of other permanents
//!   (tokens you control); no primitive grants an activated ability to a
//!   board-wide set.
//! * "Choose a Background" — GAP: not a supported `KeywordAbility` variant;
//!   the Commander Background-pairing rule is not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaheira, Friend of the Forest");
    let human = reg.interner_mut().intern("Human");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
