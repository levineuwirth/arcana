//! Gilt-Leaf Alchemist — `{G}` 1/1 Elf Druid with Deathtouch.
//! "{T}: Conjure a card named Forest onto the battlefield. Activate
//! only if two or more Elf cards are in your graveyard."
//!
//! Deathtouch is a base keyword. The activated ability is a Conjure
//! (Arena-only mechanic) which is not modeled by the engine, so the
//! whole ability is GAP'd — there is no Effect::Conjure variant.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gilt-Leaf Alchemist");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: "{T}: Conjure a card named Forest onto the battlefield."
    // Conjure is an Arena-only mechanic with no Effect::Conjure variant;
    // the graveyard-count activation precondition is also not expressible.
    reg.register(CardDefinition::new(name, chars))
}
