//! Perennial Behemoth — `{5}` 2/7 Artifact Creature — Beast.
//! "You may play lands from your graveyard." — GAP (static play-permission
//!   from the graveyard; not a triggered/activated ability and no
//!   permission Effect).
//! "Unearth {G}{G}" — GAP (the Unearth keyword is not in the usable
//!   KeywordAbility surface, and there is no return-self-from-graveyard
//!   activation primitive; follows the catalog precedent).
//!
//! Both non-bones lines are unexpressible, so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Perennial Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // GAP: "You may play lands from your graveyard." — static permission.
    // GAP: "Unearth {G}{G}" — no KeywordAbility::Unearth / graveyard self-
    // return activation.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
