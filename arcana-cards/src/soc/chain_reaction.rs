//! Chain Reaction — `{2}{R}{R}` sorcery. "Chain Reaction deals X damage to each creature, where
//! X is the number of creatures on the battlefield."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chain Reaction");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Chain Reaction deals X damage to each creature, where X is the number of creatures on the battlefield.".into(),
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
    let filter = ObjectFilter::creature();
    let x = script::count_matching(state, &filter, entry.controller) as u32;
    if x == 0 {
        return Vec::new();
    }
    let ids = script::ids_matching(state, &filter, entry.controller);
    ids.into_iter().map(|id| Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(id),
        amount: x,
    }).collect()
}
