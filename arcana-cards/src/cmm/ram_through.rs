//! Ram Through — `{1}{G}` instant, "Target creature you control deals damage
//! equal to its power to target creature you don't control. If the creature
//! you control has trample, excess damage is dealt to that creature's
//! controller instead."
//!
//! # GAP
//! - "Excess damage dealt to controller if has trample" requires a
//!   conditional replacement effect tied to the damage that is not in the
//!   catalog.
//! The damage based on the attacking creature's power is expressed;
//! the trample-excess clause is omitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ram Through");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature you don't control. If the creature you control has trample, excess damage is dealt to that creature's controller instead.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
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
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(t1) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(id_a) = t0 else { return Vec::new(); };
    let TargetChoice::Object(id_b) = t1 else { return Vec::new(); };
    let power = script::power_of(state, *id_a).max(0) as u32;
    // GAP: trample-excess damage to controller not expressible
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id_b),
        amount: power,
    }]
}
