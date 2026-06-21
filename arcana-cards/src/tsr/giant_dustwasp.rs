//! Giant Dustwasp — `{3}{G}{G}` 3/3 Insect with Flying.
//! "Flying
//!  Suspend 4—{1}{G}"
//!
//! Flying is a base keyword. Suspend is an alternative-cast/exile
//! mechanic not in the usable KeywordAbility set (only `Warp` is a
//! pre-wired marker), so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Dustwasp");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Suspend 4—{1}{G} — alternative-cast/exile mechanic not in
        // the usable KeywordAbility set.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
