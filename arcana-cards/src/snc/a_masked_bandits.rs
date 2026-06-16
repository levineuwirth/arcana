//! A-Masked Bandits — `{3}{B}{R}{G}` 5/6 Raccoon Rogue.
//! Vigilance, Menace.
//! "{1}, Exile Masked Bandits from your hand: Target land gains
//!  '{T}: Add {B}, {R}, or {G}' until Masked Bandits is cast from exile.
//!  You may cast Masked Bandits for as long as it remains exiled."
//!
//! Vigilance and Menace are base keywords. The hand-activated ability is
//! GAP'd: it would have to grant a land a triple-color mana ability whose
//! duration is "until this card is cast from exile" and also grant a
//! cast-from-exile permission — none of that linkage is expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "{1}, Exile this from hand: Target land gains '{T}: Add {B}, {R}, or
// {G}' until this is cast from exile; you may cast this while it remains
// exiled." No way to grant an arbitrary multi-color mana ability with an
// "until cast from exile" duration, nor to grant cast-from-exile permission.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Masked Bandits");
    let raccoon = reg.interner_mut().intern("Raccoon");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(raccoon);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
