//! Too Greedily, Too Deep — `{5}{B}{R}` sorcery. "Put target creature card from a
//! graveyard onto the battlefield under your control. That creature deals damage equal
//! to its power to each other creature."
//!
//! # GAP: "deals damage equal to its power to each other creature" — damage scaled by
//! the reanimated creature's power at resolution not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Too Greedily, Too Deep");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put target creature card from a graveyard onto the battlefield under your control. That creature deals damage equal to its power to each other creature.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: deal damage equal to creature's power to each other creature not expressible
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
