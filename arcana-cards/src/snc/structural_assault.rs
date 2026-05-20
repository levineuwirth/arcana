//! Structural Assault — `{3}{R}{R}` sorcery. "Destroy all artifacts,
//! then Structural Assault deals damage to each creature equal to the
//! number of artifacts that were put into graveyards from the
//! battlefield this turn."

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all artifacts, then Structural Assault deals damage to each creature equal to the number of artifacts that were put into graveyards from the battlefield this turn.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // Number of artifacts destroyed this way == artifacts on
    // battlefield right now (measured before the wipe resolves).
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let artifact_ids = script::ids_matching(state, &artifact_filter, entry.controller);
    let amount = artifact_ids.len() as u32;
    let mut effects = vec![Effect::ForEach {
        targets: artifact_ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }];
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    for c in creatures {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(c),
            amount,
        });
    }
    effects
}
