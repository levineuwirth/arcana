//! Nexos — `{1}{G}` 2/2 green Human Tyranid Advisor.
//! "Strategic Coordinator — Basic lands you control have '{T}: Add {C}{C}.
//! Spend this mana only on costs that contain {X}.'"
//!
//! GAP: This is a static ability granting abilities to other permanents
//! (basic lands). No Effect variant or trigger models static ability grants.
//! There is no `TriggeredAbilityDef` or `ActivatedAbilityDef` to express this.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nexos");
    let human = reg.interner_mut().intern("Human");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(tyranid);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: static ability granting mana abilities to basic lands not expressible
    reg.register(CardDefinition::new(name, chars))
}
