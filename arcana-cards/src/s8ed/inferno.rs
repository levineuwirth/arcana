//! Inferno — `{5}{R}{R}` instant, "Inferno deals 6 damage to each creature
//! and each player."

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
    let name = reg.interner_mut().intern("Inferno");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Inferno deals 6 damage to each creature and each player.".into(),
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
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![
        Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 6,
            }),
        },
        // GAP: deal damage to each player (no Effect for "each player"; emitting for controller only as partial)
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(entry.controller),
            amount: 6,
        },
    ]
}
