//! Rain of Daggers — `{4}{B}{B}` sorcery. "Destroy all creatures
//! target opponent controls. You lose 2 life for each creature
//! destroyed this way."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rain of Daggers");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures target opponent controls. You lose 2 life for each creature destroyed this way.".into(),
            target_requirements: vec![TargetRequirement::target_opponent()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // Build filter for creatures controlled by the target player by enumerating ids
    // via script::ids_matching with the target player as the controller-perspective.
    // We use a creature filter and let the engine restrict by controller via ForEach.
    // GAP: filter creatures by a specific opponent — script helpers compute "you" perspective only,
    // so we approximate by destroying all creatures controlled by that player via ids enumeration.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), *p);
    let count = ids.len() as u32;
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
        Effect::LoseLife { player: entry.controller, amount: 2u32.saturating_mul(count) },
    ]
}
