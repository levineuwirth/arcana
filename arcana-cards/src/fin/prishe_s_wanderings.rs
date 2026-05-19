//! Prishe's Wanderings — `{2}{G}` instant, "Search your library for a basic
//! land card or Town card, put it onto the battlefield tapped, then shuffle.
//! When you search your library this way, put a +1/+1 counter on target
//! creature you control."
//! The "Town card" subtype is not a basic land type known to `ObjectFilter`;
//! best effort searches for a land. The triggered +1/+1 counter on search
//! cannot be expressed as a triggered ability on a spell — emitted as a
//! second Effect instead.
//!
//! # GAP: Town subtype filter (non-basic Town land type not expressible)
//! # GAP: conditional triggered ability on the spell's own search (emitted as sequenced Effect)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prishe's Wanderings");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a basic land card or Town card, put it onto the battlefield tapped, then shuffle. When you search your library this way, put a +1/+1 counter on target creature you control.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
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
    use arcana_core::types::CounterKind;
    let mut effects = vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
    ];
    if let Some(target) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    effects
}
