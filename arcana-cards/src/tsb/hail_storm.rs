//! Hail Storm — `{1}{G}{G}` instant. "Hail Storm deals 2 damage to each
//! attacking creature and 1 damage to you and each creature you control."

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
    let name = reg.interner_mut().intern("Hail Storm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Hail Storm deals 2 damage to each attacking creature and 1 damage to you and each creature you control.".into(),
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
    // GAP: filter attacking creatures specifically (no attacking-creature filter in ObjectFilter)
    // Best effort: deal 1 damage to controller and each creature controller controls
    let all_creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut effects = Vec::new();
    // 1 damage to controller
    effects.push(Effect::LoseLife { player: entry.controller, amount: 1 });
    // 1 damage to each creature controller controls
    effects.push(Effect::ForEach {
        targets: all_creatures,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 1,
        }),
    });
    // GAP: 2 damage to each attacking creature (no attacking-creature filter available)
    effects
}
