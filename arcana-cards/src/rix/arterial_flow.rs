//! Arterial Flow — `{1}{B}{B}` sorcery.
//! "Each opponent discards two cards. If you control a Vampire, each opponent
//! loses 2 life and you gain 2 life."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arterial Flow");
    let _vampire = reg.interner_mut().intern("Vampire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each opponent discards two cards. If you control a Vampire, each opponent loses 2 life and you gain 2 life.".into(),
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
    let opponents = script::opponents(state, entry.controller);
    let mut effects: Vec<Effect> = opponents.iter().map(|&opp| {
        Effect::Discard { player: opp, count: 2, choice: DiscardChoice::ControllerChooses }
    }).collect();

    let vampire_filter = script::subtype_filter(reg, "Vampire")
        .controlled_by(ControllerConstraint::You);
    let vampire_count = script::count_matching(state, &vampire_filter, entry.controller);
    if vampire_count > 0 {
        for &opp in &opponents {
            effects.push(Effect::LoseLife { player: opp, amount: 2 });
        }
        effects.push(Effect::GainLife { player: entry.controller, amount: 2 });
    }
    effects
}
