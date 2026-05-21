//! Unified Strike — `{W}` instant. Exile target attacking creature if
//! its power is less than or equal to the number of Soldiers on the
//! battlefield. ("Attacking creature" filter not modeled; conditional
//! exile is emitted.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unified Strike");
    let _soldier = reg.interner_mut().intern("Soldier");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target attacking creature if its power is less than or equal to the number of Soldiers on the battlefield.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    // GAP: "attacking" predicate not expressible in the target filter — apply
    // the power-vs-Soldier-count check, but accept any creature as target.
    let soldiers = script::count_matching(
        state,
        &script::subtype_filter(reg, "Soldier"),
        entry.controller,
    );
    let power = script::power_of(state, id).max(0) as u32;
    if power <= soldiers {
        vec![Effect::ExilePermanent { target: id }]
    } else {
        Vec::new()
    }
}
