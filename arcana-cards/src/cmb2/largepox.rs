//! Largepox — `{B}{B}{B}{B}` sorcery. Each player discards, loses 1,
//! sacrifices an artifact / creature / enchantment / land /
//! planeswalker / tribal, exiles from graveyard, mills, removes a
//! counter, gets a poison counter. We emit the discard, life loss,
//! the standard sacrifice categories (artifact, creature, enchantment,
//! land, planeswalker), the mill, and ExileFromGraveyard via a chosen
//! single target — but most of the latter steps need controller-
//! choice-of-own-permanent prompts that aren't in the catalog. We
//! emit the cleanly expressible per-player sequence and GAP the rest.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Largepox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards a card, then loses 1 life, then sacrifices an artifact, a creature, an enchantment, a land, a planeswalker, and a tribal permanent, then exiles a card from their graveyard, then puts the top card of their library into their graveyard, then removes a counter from a permanent they control, then gets a poison counter.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: tribal-permanent filter, exile-a-card-from-own-graveyard
    // (we'd need a per-player target prompt), remove-counter-from-own-
    // permanent, poison counter. Emit the cleanly per-player steps.
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        effects.push(Effect::Discard { player: p, count: 1, choice: DiscardChoice::ControllerChooses });
        effects.push(Effect::LoseLife { player: p, amount: 1 });
        effects.push(Effect::Sacrifice { player: p, filter: ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()), count: 1 });
        effects.push(Effect::Sacrifice { player: p, filter: ObjectFilter::creature(), count: 1 });
        effects.push(Effect::Sacrifice { player: p, filter: ObjectFilter::permanent().with_types(TypeLine::ENCHANTMENT.into()), count: 1 });
        effects.push(Effect::Sacrifice { player: p, filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()), count: 1 });
        effects.push(Effect::Sacrifice { player: p, filter: ObjectFilter::permanent().with_types(TypeLine::PLANESWALKER.into()), count: 1 });
        effects.push(Effect::Mill { player: p, count: 1 });
    }
    effects
}
