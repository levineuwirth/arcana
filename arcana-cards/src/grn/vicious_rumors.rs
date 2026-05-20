//! Vicious Rumors — `{B}` sorcery. "Vicious Rumors deals 1 damage to
//! each opponent. Each opponent discards a card, then mills a card.
//! You gain 1 life."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vicious Rumors");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Vicious Rumors deals 1 damage to each opponent. Each opponent discards a card, then mills a card. You gain 1 life.".into(),
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
    let mut effects = Vec::new();
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(opp),
            amount: 1,
        });
        effects.push(Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
        effects.push(Effect::Mill { player: opp, count: 1 });
    }
    effects.push(Effect::GainLife { player: entry.controller, amount: 1 });
    effects
}
