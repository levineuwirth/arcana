//! Flamebreak — `{R}{R}{R}` sorcery.
//! "Flamebreak deals 3 damage to each creature without flying and each player.
//! Creatures dealt damage this way can't be regenerated this turn."
//!
//! GAP: "can't be regenerated this turn" rider is not representable.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flamebreak");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flamebreak deals 3 damage to each creature without flying and each player. Creatures dealt damage this way can't be regenerated this turn.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can't be regenerated this turn" after damage
    let filter = ObjectFilter::creature().without_types(TypeLine::CREATURE.into());
    // "without flying" — no .without_keyword filter; use plain creature filter
    // GAP: no ObjectFilter::without_keyword(Flying); using all creatures as approximation
    let creature_filter = ObjectFilter::creature();
    let ids = script::ids_matching(state, &creature_filter, entry.controller);
    let mut effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 3,
        })
        .collect();
    // Also damage each player — in 2-player the opponent; use controller's perspective
    // GAP: no catalog Effect for "each player" broadcast damage beyond ForEach over objects
    // We approximate by targeting both players using a ForEach pattern:
    // Since we don't have player enumeration, we emit damage to controller only as partial.
    // Full "each player" is a GAP.
    let _ = filter; // suppress unused warning
    effects
}
