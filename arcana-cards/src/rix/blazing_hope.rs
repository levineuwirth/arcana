//! Blazing Hope — `{W}` instant. "Exile target creature with power greater
//! than or equal to your life total."
//!
//! GAP: conditional targeting filter (power >= controller life total) is not
//! expressible with TargetFilter; the exile effect itself is modeled as
//! best effort but the targeting restriction cannot be enforced.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blazing Hope");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature with power greater than or equal to your life total.".into(),
                target_requirements: vec![
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
    // GAP: targeting restriction (power >= life total) not enforced at resolution
    let Some(t) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = t else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: *id }]
}
