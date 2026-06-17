//! Falthis, Shadowcat Familiar — `{2}{B}` 2/2 Legendary Nightmare Cat.
//! "Commanders you control have menace and deathtouch." Partner.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Falthis, Shadowcat Familiar");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Partner is not a supported KeywordAbility variant.
        ..Default::default()
    };

    // GAP: "Commanders you control have menace and deathtouch" is a static
    // continuous ability granting keywords to commanders — not a
    // triggered/activated ability and no commander-scoped grant primitive.
    reg.register(CardDefinition::new(name, chars))
}
