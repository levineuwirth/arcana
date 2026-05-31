//! Astarion's Thirst — `{3}{B}` instant. "Exile target creature. Put
//! X +1/+1 counters on a commander creature you control, where X is
//! the power of the creature exiled this way."
//!
//! The exile is faithful. The counter rider targets a SEPARATE
//! permanent ("a commander creature you control") chosen at
//! resolution, with X equal to the power of the now-exiled creature —
//! there is no second target slot, no "commander creature you control"
//! selector, and no way to recover the exiled creature's power after
//! it has left the battlefield. That clause is GAPped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Astarion's Thirst");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature. Put X +1/+1 counters on a commander creature you control, where X is the power of the creature exiled this way.".into(),
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
    vec![Effect::ExilePermanent { target: *id }]
    // GAP: "Put X +1/+1 counters on a commander creature you control,
    // where X is the power of the creature exiled this way." Requires a
    // separate "commander creature you control" selection (no second
    // target, no commander-creature filter) and X = the power of the
    // creature once it is already in exile (lost-information amount).
}
