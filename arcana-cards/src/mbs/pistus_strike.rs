//! Pistus Strike — `{2}{G}` Instant. "Destroy target creature with
//! flying. That creature's controller gets a poison counter."
//!
//! # Implementation note
//! DestroyPermanent is expressible. Poison counters on a player are
//! modeled via CounterKind but there is no Effect variant for giving a
//! counter to a player (AddCounters targets an ObjectId, not a
//! PlayerId).
//!
//! # GAP
//! No Effect for adding poison counter to a player (AddCounters targets
//! objects, not players); targeting filter restricted to flying
//! creatures not available.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pistus Strike");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature with flying. That creature's controller gets a poison counter.".into(),
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
    vec![
        Effect::DestroyPermanent { target: *id },
        // GAP: no Effect for giving poison counter to a player
        // GAP: target filter for flying creatures not available
    ]
}
