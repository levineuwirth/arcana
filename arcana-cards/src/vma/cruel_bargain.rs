//! Cruel Bargain — `{B}{B}{B}` sorcery, "Draw four cards. You lose half your
//! life, rounded up."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Bargain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw four cards. You lose half your life, rounded up.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let life = script::life(state, entry.controller);
    let amount = ((life + 1) / 2).max(0) as u32;
    vec![
        Effect::DrawCards { player: entry.controller, count: 4 },
        Effect::LoseLife { player: entry.controller, amount },
    ]
}
