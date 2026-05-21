//! Burn the Accursed — `{4}{R}` instant. "Burn the Accursed deals 5
//! damage to target creature and 2 damage to that creature's
//! controller. If that creature would die this turn, exile it
//! instead."

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burn the Accursed");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Burn the Accursed deals 5 damage to target creature and 2 damage to that creature's controller. If that creature would die this turn, exile it instead.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let controller = script::target_controller(state, *id, entry.controller);
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 5,
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(controller),
            amount: 2,
        },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::ThisDies,
            action: DelayedAction::Exile,
        },
    ]
}
