//! A-Nael, Avizoa Aeronaut — `{G}{U}` 2/2 Legendary Elf Scout with Flying.
//! Domain combat-damage trigger (Domain-scaled library look + conditional
//! draw) is unexpressible; only Flying is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Nael, Avizoa Aeronaut");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Domain combat-damage trigger — "look at the top X cards where X is
    // the number of basic land types among lands you control, put up to one on
    // top and the rest on the bottom, then if five basic land types draw a
    // card." No domain-count script helper and no "look, reorder top-vs-bottom"
    // effect; whole ability unexpressible.
    reg.register(CardDefinition::new(name, chars))
}
