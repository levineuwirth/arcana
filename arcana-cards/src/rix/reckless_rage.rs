//! Reckless Rage — `{R}` instant. "Reckless Rage deals 4 damage to target creature you don't
//! control and 2 damage to target creature you control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Rage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Reckless Rage deals 4 damage to target creature you don't control and 2 damage to target creature you control.".into(),
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
    let mut targets = entry.targets.targets.iter();
    let Some(TargetChoice::Object(enemy)) = targets.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(own)) = targets.next() else { return Vec::new(); };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*enemy),
            amount: 4,
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*own),
            amount: 2,
        },
    ]
}
