//! Syphon Soul — `{2}{B}` sorcery. "Syphon Soul deals 2 damage to each
//! other player. You gain life equal to the damage dealt this way."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syphon Soul");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Syphon Soul deals 2 damage to each other player. You gain life equal to the damage dealt this way.".into(),
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
    let opps = script::opponents(state, entry.controller);
    let total = (opps.len() as u32) * 2;
    let mut effects: Vec<Effect> = opps
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 2,
        })
        .collect();
    if total > 0 {
        effects.push(Effect::GainLife {
            player: entry.controller,
            amount: total,
        });
    }
    effects
}
