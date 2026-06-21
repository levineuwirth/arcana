//! Observant Alseid — `{2}{W}` 2/2 Enchantment Creature — Nymph.
//!
//! Oracle:
//! * Bestow {4}{W}.
//! * Vigilance.
//! * Enchanted creature gets +2/+2 and has vigilance.
//!
//! Only the Vigilance keyword is expressible here. Bestow is not in the
//! usable `KeywordAbility` set (and the bestow/aura-attach machinery is a
//! separate card shape), so it is GAP'd along with the enchanted-creature
//! buff static.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Observant Alseid");
    let nymph = reg.interner_mut().intern("Nymph");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nymph);

    // GAP: keyword — Bestow {4}{W} is not in the usable KeywordAbility set.
    // GAP: static — "Enchanted creature gets +2/+2 and has vigilance"
    // (Aura/bestow attach buff; separate card shape).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
