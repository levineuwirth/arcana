//! Chatterfang, Squirrel General — `{2}{G}` 3/3 Legendary Squirrel Warrior.
//! Forestwalk.
//! If one or more tokens would be created under your control, those
//! tokens plus that many 1/1 green Squirrel tokens are created instead.
//! (GAP'd — token-creation replacement effect, no expressible hook.)
//! {B}, Sacrifice X Squirrels: Target creature gets +X/-X until end of
//! turn. (GAP'd — variable-X sacrifice count is not expressible, and no
//! activated dynamic-X reads the count for the +X/-X amount.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chatterfang, Squirrel General");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warrior = reg.interner_mut().intern("Warrior");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };
    // GAP: token-creation replacement (extra Squirrel per token) is a
    // static replacement effect, not expressible.
    // GAP: "{B}, Sacrifice X Squirrels: target creature gets +X/-X" —
    // the variable-X sacrifice count and X-scaled pump have no
    // expressible activated-ability form.
    reg.register(CardDefinition::new(name, chars))
}
