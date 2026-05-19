//! Chandra's Revolution — `{3}{R}` sorcery. Chandra's Revolution deals 4
//! damage to target creature. Tap target land. That land doesn't untap during
//! its controller's next untap step.
//!
//! GAP: "doesn't untap during its controller's next untap step" effect not
//! in catalog.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra's Revolution");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Chandra's Revolution deals 4 damage to target creature. Tap target land. That land doesn't untap during its controller's next untap step.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
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
    let Some(creature_target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(creature_id) = creature_target else { return Vec::new(); };
    let Some(land_target) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(land_id) = land_target else { return Vec::new(); };
    // GAP: "doesn't untap next untap step" effect not expressible
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*creature_id),
            amount: 4,
        },
        Effect::Tap { target: *land_id },
    ]
}
