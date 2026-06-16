//! Ghalta, Primal Hunger — `{10}{G}{G}` 12/12 Legendary Elder Dinosaur
//! with Trample. Costs {X} less to cast where X is the total power of
//! creatures you control.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghalta, Primal Hunger");
    let elder = reg.interner_mut().intern("Elder");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dinosaur);

    // GAP: static cost-reduction "costs {X} less to cast, where X is the
    // total power of creatures you control" — no cost-reduction primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(12)),
        toughness: Some(PtValue::Fixed(12)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
