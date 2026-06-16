//! Cradle Guard — `{1}{G}{G}` 4/4 Treefolk.
//! Trample. "Echo {1}{G}{G}."
//!
//! Trample is a base keyword. Echo is not an available KeywordAbility
//! variant and its upkeep sacrifice-unless-pay machinery is not
//! expressible here, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: Echo {1}{G}{G} — no Echo KeywordAbility variant and no way to model
// the "first-upkeep-after-arrival: sacrifice unless you pay the echo cost"
// trigger; emitted as bones + Trample only.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cradle Guard");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
