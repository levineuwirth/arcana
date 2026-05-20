//! Skull Rend — `{3}{B}{R}` sorcery. "Skull Rend deals 2 damage to
//! each opponent. Those players each discard two cards at random."

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
    let name = reg.interner_mut().intern("Skull Rend");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Skull Rend deals 2 damage to each opponent. Those players each discard two cards at random.".into(),
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
    let mut out = Vec::new();
    for p in script::opponents(state, entry.controller) {
        out.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 2,
        });
        out.push(Effect::Discard {
            player: p,
            count: 2,
            choice: DiscardChoice::Random,
        });
    }
    out
}
