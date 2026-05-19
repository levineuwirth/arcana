//! Sulfurous Blast — `{2}{R}{R}` instant. "Sulfurous Blast deals 2 damage to
//! each creature and each player. If you cast this spell during your main phase,
//! Sulfurous Blast deals 3 damage to each creature and each player instead."
//!
//! GAP: "if cast during your main phase" conditional (requires phase-check at
//! cast/resolution time, not available via script:: helpers). Emitting the
//! unconditional 2-damage version.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sulfurous Blast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Sulfurous Blast deals 2 damage to each creature and each player. If you cast this spell during your main phase, Sulfurous Blast deals 3 damage to each creature and each player instead.".into(),
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
    // GAP: conditional main-phase check for 3 damage vs 2 damage
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: creature_ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 2,
        }),
    }]
    // GAP: also deals 2 damage to each player (no per-opponent loop in Effect API)
}
