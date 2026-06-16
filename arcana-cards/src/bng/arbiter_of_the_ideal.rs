//! Arbiter of the Ideal — `{4}{U}{U}` 4/5 Sphinx with Flying.
//! Inspired — Whenever this creature becomes untapped, reveal the top card
//! of your library; if it's an artifact, creature, or land card, you may
//! put it onto the battlefield with a manifestation counter on it as an
//! enchantment in addition to its other types.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arbiter of the Ideal");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    // GAP: Inspired — no "becomes untapped" TriggerCondition variant, and no
    // reveal-top-then-may-put-onto-battlefield-with-counter effect.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
