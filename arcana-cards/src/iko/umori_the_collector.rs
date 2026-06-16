//! Umori, the Collector — `{2}{B/G}{B/G}` 4/5 Legendary Ooze.
//! Companion (GAP — companion deck-building condition is not modeled).
//! "As Umori enters, choose a card type." (GAP — no choose-card-type effect.)
//! "Spells you cast of the chosen type cost {1} less to cast." (GAP — no
//! cost-reduction primitive.)
//! All non-keyword text is unexpressible; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Umori, the Collector");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
