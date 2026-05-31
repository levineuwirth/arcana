//! Black Hole — `{3}{B}` sorcery. "Destroy target creature and up to X
//! other target creatures, where X is the number of Attractions you've
//! visited this turn."
//!
//! The mandatory single target is destroyed. The variable additional
//! targets ("up to X other target creatures", X = Attractions visited
//! this turn) are a dynamic, Un-set Attraction-count mechanic with no
//! target-spec or `script::*` support, so they are GAP-ed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Black Hole");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature and up to X other target creatures, where X is the number of Attractions you've visited this turn.".into(),
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
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "and up to X other target creatures, where X is the number of
    // Attractions you've visited this turn" — variable target count keyed
    // off Attractions-visited (Un-set mechanic) is not expressible in the
    // target spec, and there is no script:: helper for Attraction count.
    vec![Effect::DestroyPermanent { target: *id }]
}
