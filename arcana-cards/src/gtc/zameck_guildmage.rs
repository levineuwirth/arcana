//! Zameck Guildmage — `{G}{U}` 2/2 Elf Wizard.
//! "{G}{U}: This turn, each creature you control enters with an additional
//! +1/+1 counter on it."
//! "{G}{U}, Remove a +1/+1 counter from a creature you control: Draw a card."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zameck Guildmage");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "{G}{U}: This turn, each creature you control enters with an
    // additional +1/+1 counter" — installs an enters-with-counters replacement
    // for the turn; no Effect primitive for a turn-scoped enters-with rider.
    // GAP: "{G}{U}, Remove a +1/+1 counter from a creature you control: Draw a
    // card" — the activation cost "remove a counter from a CHOSEN OTHER
    // creature" has no ActivationCost field (only remove_self_counter exists).
    reg.register(CardDefinition::new(name, chars))
}
