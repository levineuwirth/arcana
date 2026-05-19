//! March of Souls — `{4}{W}` sorcery.
//! "Destroy all creatures. They can't be regenerated. For each creature destroyed this way, its controller creates a 1/1 white Spirit creature token with flying."
//! GAP: per-controller token creation after board wipe (each creature's controller creates a token) is not expressible.
//! Best effort: destroy all creatures; token creation omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("March of Souls");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. They can't be regenerated. For each creature destroyed this way, its controller creates a 1/1 white Spirit creature token with flying.".into(),
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
    // GAP: per-controller token creation based on destroyed creatures is not expressible
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: arcana_core::objects::NULL_OBJECT_ID }),
    }]
}
