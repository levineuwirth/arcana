//! Malicious Eclipse — `{1}{B}{B}` sorcery. "All creatures get -2/-2
//! until end of turn. If a creature an opponent controls would die
//! this turn, exile it instead."
//!
//! The team-wide -2/-2 is expressed; the die-replacement rider has
//! no primitive.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malicious Eclipse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "All creatures get -2/-2 until end of turn. If a creature an opponent controls would die this turn, exile it instead.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut out = Vec::new();
    for id in ids {
        out.push(Effect::Pump {
            target: id,
            power: -2,
            toughness: -2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    // GAP: "if an opponent's creature would die this turn, exile it
    // instead" — no death-replacement primitive.
    out
}
