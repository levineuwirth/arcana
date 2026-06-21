//! Orgg — `{3}{R}{R}` 6/6 red Orgg with Trample.
//! "This creature can't attack if defending player controls an untapped
//! creature with power 3 or greater." (GAP — static attack restriction.)
//! "This creature can't block creatures with power 3 or greater." (GAP —
//! static block restriction.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orgg");
    let orgg = reg.interner_mut().intern("Orgg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orgg);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "can't attack if defending player controls an untapped creature with
    // power 3 or greater" — a conditional static attack restriction is not
    // expressible as a triggered/activated ability.
    // GAP: "can't block creatures with power 3 or greater" — a static block
    // restriction is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
