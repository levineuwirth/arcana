//! Structural Assault — `{3}{R}{R}` sorcery. "Destroy all artifacts, then
//! Structural Assault deals damage to each creature equal to the number of
//! artifacts that were put into graveyards from the battlefield this turn."
//!
//! # GAP: tracking how many artifacts were destroyed (went to graveyard) during
//!   this resolution — no script helper for "permanents that died this turn"
//! Best-effort: count artifacts on the battlefield before destroying them, then
//! use that count for the damage.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Structural Assault");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all artifacts, then Structural Assault deals damage to each creature equal to the number of artifacts that were put into graveyards from the battlefield this turn.".into(),
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
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let artifacts = script::ids_matching(state, &artifact_filter, entry.controller);
    let artifact_count = artifacts.len() as u32;
    let mut effects: Vec<Effect> = vec![Effect::ForEach {
        targets: artifacts,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    if artifact_count > 0 {
        let creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
        for id in creatures {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(id),
                amount: artifact_count,
            });
        }
    }
    effects
}
