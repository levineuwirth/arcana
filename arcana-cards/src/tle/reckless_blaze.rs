//! Reckless Blaze — `{3}{R}{R}` sorcery — Lesson, "Reckless Blaze deals 5 damage to each
//! creature. Whenever a creature you controlled that was dealt damage this way dies this turn,
//! add {R}."
//!
//! GAP: Lesson subtype (not in TypeLine constants).
//! GAP: Triggered add-mana on death of creatures dealt damage this way (delayed triggered
//! ability with mana production not in Effect catalog).
//! GAP: Enumerate all creatures on the battlefield to populate ForEach targets
//! (no documented state accessor in resolver signature).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Blaze");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Reckless Blaze deals 5 damage to each creature. Whenever a creature you controlled that was dealt damage this way dies this turn, add {R}.".into(),
                target_requirements: vec![],
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
    // GAP: no documented API to enumerate all battlefield creatures from resolver
    // GAP: triggered mana production on creature death not in Effect catalog
    vec![Effect::ForEach {
        targets: vec![],
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 5,
        }),
    }]
}
