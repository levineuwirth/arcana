//! Grisly Spectacle — `{2}{B}{B}` instant, "Destroy target nonartifact
//! creature. Its controller mills cards equal to that creature's power."
//!
//! GAP: "nonartifact creature" filter — ObjectFilter has no without_types
//! for ARTIFACT on a creature target; "mill equal to creature's power" —
//! dynamic mill count not supported (Effect::Mill requires a fixed u32).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grisly Spectacle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target nonartifact creature. Its controller mills cards equal to that creature's power.".into(),
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
    // GAP: nonartifact creature filter (no without_types(ARTIFACT) on creature
    // target); mill equal to creature's power (dynamic count)
    vec![Effect::DestroyPermanent { target: *id }]
}
