//! Arterial Flow — `{1}{B}{B}` sorcery. "Each opponent discards two cards. If
//! you control a Vampire, each opponent loses 2 life and you gain 2 life."
//!
//! GAP: "each opponent" iteration — no multi-player opponent iteration; using
//! TargetPlayer for single opponent as best-effort.
//! GAP: "if you control a Vampire" — script::subtype_filter + count_matching
//! can check this, but multi-opponent iteration still missing.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
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
                // GAP: no multi-player "each opponent" iteration; using single target player
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(opp) = target else { return Vec::new(); };
    let mut effects = vec![
        Effect::Discard { player: *opp, count: 2, choice: DiscardChoice::ControllerChooses },
    ];
    let vampire_filter = script::subtype_filter(reg, "Vampire")
        .controlled_by(ControllerConstraint::You);
    let vampire_count = script::count_matching(state, &vampire_filter, entry.controller);
    if vampire_count > 0 {
        effects.push(Effect::LoseLife { player: *opp, amount: 2 });
        effects.push(Effect::GainLife { player: entry.controller, amount: 2 });
    }
    effects
}
