//! Poison the Well — `{2}{B/R}{B/R}` sorcery. "Destroy target land. Poison the Well deals 2
//! damage to that land's controller."
//!
//! GAP: After destroying the land we need to deal damage to that land's controller; no API to
//! look up the controller of a targeted permanent. Using entry.controller as approximation.

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
    let name = reg.interner_mut().intern("Poison the Well");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target land. Poison the Well deals 2 damage to that land's controller.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: cannot look up controller of the targeted land; using entry.controller as approximation
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(entry.controller),
            amount: 2,
        },
    ]
}
