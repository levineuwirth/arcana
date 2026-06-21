//! Glade Watcher — `{1}{G}` 3/3 Elemental with Defender.
//! "Formidable — {G}: This creature can attack this turn as though it
//! didn't have defender. Activate only if creatures you control have
//! total power 8 or greater."
//!
//! Defender is a base keyword. The Formidable activated ability's effect
//! ("can attack this turn as though it didn't have defender") has no
//! expressible Effect variant, and the "total power 8 or greater"
//! activation gate has no script/conditions helper for summing power, so
//! the whole ability is GAP'd.

// GAP (activated ability): "{G}: This creature can attack this turn as
// though it didn't have defender. Activate only if creatures you control
// have total power 8 or greater." — no Effect expresses attack-permission
// despite Defender, and no helper sums total power for the Formidable gate.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glade Watcher");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
