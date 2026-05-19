//! Blood Lust — `{1}{R}` instant.
//! "If target creature has toughness 5 or greater, it gets +4/-4 until end of
//! turn. Otherwise, it gets +4/-X until end of turn, where X is its toughness
//! minus 1."
//!
//! # GAP: conditional Pump based on target's toughness; dynamic toughness
//! penalty (X = toughness - 1)
//! The catalog has no mechanism to read a permanent's toughness at resolve
//! time and derive a dynamic pump value from it.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blood Lust");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "If target creature has toughness 5 or greater, it gets +4/-4 until end of turn. Otherwise, it gets +4/-X until end of turn, where X is its toughness minus 1.".into(),
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
    // GAP: read target's toughness to determine conditional pump values
    // Best effort: apply the simple +4/-4 case (toughness >= 5 branch)
    vec![Effect::Pump {
        target: *id,
        power: 4,
        toughness: -4,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
