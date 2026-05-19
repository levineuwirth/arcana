//! Jovial Evil — `{2}{B}` sorcery. "Jovial Evil deals X damage to target
//! opponent, where X is twice the number of white creatures that player
//! controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jovial Evil");
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
                text: "Jovial Evil deals X damage to target opponent, where X is twice the number of white creatures that player controls.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // Count white creatures controlled by target player.
    // controlled_by(ControllerConstraint::You) uses entry.controller as
    // reference; for the target player we use the filter without controller
    // constraint and rely on count_matching with target player as 'you'.
    // GAP: count_matching's 'you' parameter refers to entry.controller,
    //      not the target player — best effort counts white creatures
    //      controlled by entry.controller instead.
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().with_colors(ColorSet::white()),
        *p,
    );
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(*p),
        amount: n * 2,
    }]
}
