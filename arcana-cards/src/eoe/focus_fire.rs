//! Focus Fire — `{W}` instant, "Focus Fire deals X damage to target attacking
//! or blocking creature, where X is 2 plus the number of creatures and/or
//! Spacecraft you control."
//!
//! # GAP
//! - "Attacking or blocking creature" filter is not available; using plain
//!   `target_creature()` instead.
//! - "Spacecraft" is a non-creature artifact subtype not modeled; counted
//!   creatures only via `ObjectFilter::creature()`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement, ObjectFilter, ControllerConstraint};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Focus Fire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Focus Fire deals X damage to target attacking or blocking creature, where X is 2 plus the number of creatures and/or Spacecraft you control.".into(),
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
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    // GAP: Spacecraft (non-creature artifact subtype) not counted; attacking/blocking filter not available
    let amount = 2 + n;
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
