//! Searing Blood — `{R}{R}` instant.
//! "Searing Blood deals 2 damage to target creature. When that creature dies
//! this turn, Searing Blood deals 3 damage to the creature's controller."

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Searing Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Searing Blood deals 2 damage to target creature. When that creature dies this turn, Searing Blood deals 3 damage to the creature's controller.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // The "deals 3 damage to the creature's controller" on-death rider cannot be
    // expressed exactly: DelayedAction supports Sacrifice/Exile/ReturnToHand actions,
    // not DealDamage. GAP: on-dies DealDamage to former controller not in DelayedAction catalog.
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 2,
        },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::ThisDies,
            action: DelayedAction::Sacrifice,
        },
        // GAP: on-dies trigger should deal 3 damage to creature's controller, not sacrifice.
        //      No DealDamage action exists in DelayedAction.
    ]
}
