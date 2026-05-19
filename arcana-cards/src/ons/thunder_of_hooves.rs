//! Thunder of Hooves — `{3}{R}` sorcery. "Thunder of Hooves deals X damage to
//! each creature without flying and each player, where X is the number of Beasts
//! on the battlefield."

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
    let name = reg.interner_mut().intern("Thunder of Hooves");
    let _beast = reg.interner_mut().intern("Beast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Thunder of Hooves deals X damage to each creature without flying and each player, where X is the number of Beasts on the battlefield.".into(),
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
    let x = script::count_matching(state, &script::subtype_filter(reg, "Beast"), entry.controller) as u32;
    if x == 0 {
        return Vec::new();
    }
    // Deal x damage to each creature without flying (GAP: no "without flying" filter)
    // and each player. Using ForEach on all creatures as best effort.
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut effects: Vec<Effect> = creature_ids.into_iter().map(|id| {
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: x,
        }
    }).collect();
    // GAP: filtering creatures "without flying"; all creatures are targeted
    // Deal x damage to each player — only the two players known at resolve time
    // GAP: no way to enumerate all players; using entry.controller as placeholder
    effects.push(Effect::LoseLife { player: entry.controller, amount: x });
    effects
}
