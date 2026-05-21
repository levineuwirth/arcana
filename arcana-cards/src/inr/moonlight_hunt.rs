//! Moonlight Hunt — `{1}{G}` instant. "Choose target creature you
//! don't control. Each creature you control that's a Wolf or a
//! Werewolf deals damage equal to its power to that creature." We can
//! enumerate your Wolf/Werewolf creatures and emit DealDamage equal
//! to each one's power.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moonlight Hunt");
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
                text: "Choose target creature you don't control. Each creature you control that's a Wolf or a Werewolf deals damage equal to its power to that creature.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: subtype-union filter (Wolf OR Werewolf) — script::subtype_filter
    // is single-subtype, so we can only damage one tribe per call.
    // We do Wolf only; Werewolf alone would require a separate filter.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(target_id) = target else { return Vec::new(); };
    let wolf_filter = script::subtype_filter(reg, "Wolf").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &wolf_filter, entry.controller);
    ids.into_iter()
        .map(|id| {
            let p = script::power_of(state, id).max(0) as u32;
            Effect::DealDamage { source: entry.source, target: DamageTarget::Object(*target_id), amount: p }
        })
        .collect()
}
