//! Nyxborn Hydra — `{X}{G}` 0/1 Enchantment Creature — Hydra with Reach and
//! Trample. Bestow `{X}{G}{G}`.
//! "This permanent enters with X +1/+1 counters on it."
//! "Enchanted creature gets +1/+1 for each +1/+1 counter on this Aura and has
//! reach and trample."
//!
//! Reach and Trample are base characteristics. Bestow is not a usable keyword
//! (the bestow-as-Aura cast/attach machinery is not in the documented surface).
//! The enters-with-X-counters clause is GAP'd: the cast-time X is not readable
//! from a PendingTrigger (no x_value accessor). The bestow Aura static
//! (enchanted creature gets +1/+1 per counter + reach/trample) is a continuous
//! static, also GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyxborn Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Bestow — not a usable keyword; bestow-as-Aura cast/attach is not
        // in the documented surface.
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "enters with X +1/+1 counters" — cast-time X is not readable from a
    // PendingTrigger (no x_value accessor).
    // GAP: bestow Aura static (enchanted creature gets +1/+1 per counter and has
    // reach/trample) — continuous static, not expressible.
    reg.register(CardDefinition::new(name, chars))
}
