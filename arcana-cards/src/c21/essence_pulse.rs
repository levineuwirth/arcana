//! Essence Pulse — `{3}{B}` sorcery. "You gain 2 life. Each creature
//! gets -X/-X until end of turn, where X is the amount of life you
//! gained this turn."
//!
//! After the GainLife of 2, "life gained this turn" is at least 2; the
//! engine has no "life gained this turn" tracker, so X is approximated
//! by this spell's own 2-life gain and applied as -2/-2 to every
//! creature via ForEach over all creatures.
//!
//! GAP: X uses only this spell's life gain, not other life gained this
//! turn (no life-gained-this-turn helper).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Essence Pulse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You gain 2 life. Each creature gets -X/-X until end of turn, where X is the amount of life you gained this turn.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // GAP: X approximated by this spell's 2-life gain (no life-gained-this-turn helper).
    vec![
        Effect::GainLife { player: entry.controller, amount: 2 },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: -2,
                toughness: -2,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
    ]
}
