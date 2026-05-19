//! Sudden Disappearance — `{5}{W}` sorcery. "Exile all nonland permanents
//! target player controls. Return the exiled cards to the battlefield
//! under their owner's control at the beginning of the next end step."
//!
//! GAP: 'return exiled cards at beginning of next end step' delayed
//! trigger not expressible. Exiles all nonland permanents of target player
//! without the return rider.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sudden Disappearance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all nonland permanents target player controls. Return the exiled cards to the battlefield under their owner's control at the beginning of the next end step.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    // GAP: return at next end step not expressible
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
        entry.controller,
    );
    if ids.is_empty() { return Vec::new(); }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
