//! Plague Spores — `{4}{B}{R}` sorcery. "Destroy target nonblack creature
//! and target land. They can't be regenerated."
//!
//! # GAP: "nonblack" creature filter not available on ObjectFilter.
//! # GAP: "can't be regenerated" flag not expressible.
//! Targeting a creature and a land; destroying both.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plague Spores");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target nonblack creature and target land. They can't be regenerated.".into(),
                // GAP: nonblack creature filter not available; using general creature target
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::new().with_types(TypeLine::LAND.into())),
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
    // GAP: "can't be regenerated" not expressible
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::DestroyPermanent { target: *id })
        } else {
            None
        }
    }).collect()
}
