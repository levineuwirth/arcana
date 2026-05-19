//! Certain Death — `{5}{B}` sorcery, "Destroy target creature. Its controller
//! loses 2 life and you gain 2 life."
//!
//! GAP: "its controller loses 2 life" — accessing the target creature's
//! controller as a player at resolve time is not supported (no
//! Effect::LoseLife { player: target.controller } path in the catalog).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Certain Death");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. Its controller loses 2 life and you gain 2 life.".into(),
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
    // GAP: target creature's controller loses 2 life (no way to get target's
    // controller as a PlayerId from StackEntry)
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::GainLife { player: entry.controller, amount: 2 },
    ]
}
