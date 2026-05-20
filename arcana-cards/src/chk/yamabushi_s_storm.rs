//! Yamabushi's Storm — `{1}{R}` sorcery, "Yamabushi's Storm deals 1
//! damage to each creature. If a creature dealt damage this way would
//! die this turn, exile it instead." The die→exile replacement is not
//! expressible; the spread damage is.

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
    let name = reg.interner_mut().intern("Yamabushi's Storm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Yamabushi's Storm deals 1 damage to each creature. If a \
                   creature dealt damage this way would die this turn, \
                   exile it instead."
                .into(),
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
    // GAP: "if it would die this turn, exile it instead" replacement is
    // not expressible.
    let ids =
        script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    ids.into_iter()
        .map(|id| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 1,
        })
        .collect()
}
