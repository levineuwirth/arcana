//! Storm's Wrath — `{2}{R}{R}` sorcery. "Storm's Wrath deals 4 damage to
//! each creature and each planeswalker."
//!
//! GAP: planeswalkers are not a separate ObjectFilter type; only creatures
//! are targeted here. Deals 4 damage to each creature.

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
    let name = reg.interner_mut().intern("Storm's Wrath");
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
                text: "Storm's Wrath deals 4 damage to each creature and each planeswalker.".into(),
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
    // GAP: planeswalker type filter not expressible; creatures only
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    if ids.is_empty() { return Vec::new(); }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 4,
        }),
    }]
}
