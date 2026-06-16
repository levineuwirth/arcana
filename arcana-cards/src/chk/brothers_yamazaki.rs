//! Brothers Yamazaki — `{2}{R}` 2/1 Legendary Human Samurai with Bushido 1.
//! The legend-rule exception and the static "each other creature named
//! Brothers Yamazaki gets +2/+2 and has haste" are continuous statics with no
//! triggered/activated form to express, so they are GAP'd. Keyword only.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brothers Yamazaki");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bushido(1)],
        ..Default::default()
    };

    // GAP: "If there are exactly two permanents named Brothers Yamazaki on the
    // battlefield, the legend rule doesn't apply to them" — static rule
    // exception, not expressible.
    // GAP: "Each other creature named Brothers Yamazaki gets +2/+2 and has
    // haste" — continuous static anthem, no triggered/activated form.

    reg.register(CardDefinition::new(name, chars))
}
