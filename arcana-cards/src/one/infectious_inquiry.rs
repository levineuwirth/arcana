//! Infectious Inquiry — `{2}{B}` sorcery. "You draw two cards and you
//! lose 2 life. Each opponent gets a poison counter."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infectious Inquiry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You draw two cards and you lose 2 life. Each opponent gets a poison counter.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // "You draw two cards and you lose 2 life. Each opponent gets a poison counter."
    let mut effects = vec![
        Effect::DrawCards {
            player: entry.controller,
            count: 2,
        },
        Effect::LoseLife {
            player: entry.controller,
            amount: 2,
        },
    ];
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::GivePlayerCounters {
            player: opp,
            kind: CounterKind::Poison,
            count: 1,
        });
    }
    effects
}
