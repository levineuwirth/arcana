//! Unmoored Ego — `{1}{U}{B}` sorcery. "Choose a card name. Search target
//! opponent's graveyard, hand, and library for up to four cards with that
//! name and exile them. That player shuffles, then draws a card for each
//! card exiled from their hand this way."
//!
//! Implemented via [`Effect::NameCardAndExile`] (chooser = you, target =
//! the target opponent): the engine deterministically names the most-copied
//! card in the target's hand+graveyard+library, exiles every copy, and
//! shuffles. This is the Lost Legacy / Memoricide disruption family.
//!
//! GAP: the printed "draws a card for each card exiled from their hand this
//! way" rider is not expressible — the primitive returns no per-zone exile
//! count, and the up-to-four cap is unbounded by the engine (every copy is
//! removed).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unmoored Ego");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose a card name. Search target opponent's graveyard, hand, \
                   and library for up to four cards with that name and exile them. \
                   That player shuffles, then draws a card for each card exiled \
                   from their hand this way."
                .into(),
            target_requirements: vec![TargetRequirement::target_opponent()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::NameCardAndExile {
        chooser: entry.controller,
        target: *p,
    }]
}
