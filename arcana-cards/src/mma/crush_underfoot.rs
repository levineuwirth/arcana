//! Crush Underfoot — `{1}{R}` Kindred Instant — Giant. "Choose a Giant
//! creature you control. It deals damage equal to its power to target
//! creature."
//! The Kindred type line (Kindred Instant) is not representable as a TypeLine
//! const. Best effort uses INSTANT. The "choose a Giant you control" selector
//! and "it deals damage equal to its power" delegation are approximated using
//! the first Giant found via subtype_filter.
//!
//! # GAP: Kindred type line (no TypeLine::KINDRED constant)
//! # GAP: player-chooses-which-giant-attacks (best effort: first matching Giant)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crush Underfoot");
    let _giant = reg.interner_mut().intern("Giant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        // GAP: Kindred type line (no TypeLine::KINDRED constant)
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose a Giant creature you control. It deals damage equal to its power to target creature.".into(),
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
    let TargetChoice::Object(target_id) = target else { return Vec::new(); };
    // GAP: player-chooses-which-giant-attacks (best effort: first matching Giant)
    let giant_filter = script::subtype_filter(reg, "Giant")
        .controlled_by(ControllerConstraint::You);
    let giants = script::ids_matching(state, &giant_filter, entry.controller);
    let Some(&giant_id) = giants.first() else { return Vec::new(); };
    let power = script::power_of(state, giant_id).max(0) as u32;
    if power == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*target_id),
        amount: power,
    }]
}
