//! Breath Weapon — `{2}{R}` instant. "Breath Weapon deals 2 damage to
//! each non-Dragon creature."

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
    let name = reg.interner_mut().intern("Breath Weapon");
    let _dragon = reg.interner_mut().intern("Dragon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Breath Weapon deals 2 damage to each non-Dragon creature.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let all = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let dragons: std::collections::HashSet<_> = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Dragon"),
        entry.controller,
    )
    .into_iter()
    .collect();
    let mut effects = Vec::new();
    for id in all {
        if dragons.contains(&id) {
            continue;
        }
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 2,
        });
    }
    effects
}
