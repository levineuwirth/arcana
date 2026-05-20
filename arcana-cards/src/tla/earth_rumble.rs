//! Earth Rumble — `{3}{G}` sorcery. "Earthbend 2. When you do, up to
//! one target creature you control fights target creature an opponent
//! controls."
//!
//! Earthbend (turn a land into a creature with counters and a
//! return-on-death trigger) is not expressible with the demonstrated
//! API. Only the fight is emitted, with a controlled creature and an
//! opponent's creature targeted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earth Rumble");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Earthbend 2. When you do, up to one target creature you control fights target creature an opponent controls.".into(),
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
    let Some(TargetChoice::Object(a)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(b)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    // GAP: Earthbend 2 (land becomes counters'd creature with return trigger) not expressible.
    vec![Effect::Fight { a: *a, b: *b }]
}
