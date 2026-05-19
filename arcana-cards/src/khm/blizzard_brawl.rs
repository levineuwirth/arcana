//! Blizzard Brawl — `{G}` snow sorcery. "Choose target creature you control and target
//! creature you don't control. If you control three or more snow permanents, the creature
//! you control gets +1/+0 and gains indestructible until end of turn. Then those creatures
//! fight each other."
//!
//! # GAP: "snow permanent" count condition at resolve time not expressible.
//! Fight is expressible; conditional pump with Indestructible omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blizzard Brawl");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control and target creature you don't control. If you control three or more snow permanents, the creature you control gets +1/+0 and gains indestructible until end of turn. Then those creatures fight each other.".into(),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: conditional pump/indestructible based on snow permanent count not expressible
    let targets = &entry.targets.targets;
    let (Some(t0), Some(t1)) = (targets.get(0), targets.get(1)) else { return Vec::new(); };
    let (TargetChoice::Object(id0), TargetChoice::Object(id1)) = (t0, t1) else { return Vec::new(); };
    vec![Effect::Fight { a: *id0, b: *id1 }]
}
