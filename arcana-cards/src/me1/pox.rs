//! Pox — `{B}{B}{B}` sorcery. "Each player loses a third of their
//! life, then discards a third of the cards in their hand, then
//! sacrifices a third of the creatures they control of their choice,
//! then sacrifices a third of the lands they control of their choice.
//! Round up each time." Iterates each player and emits the four
//! computed rounded-up thirds using script::* helpers.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player loses a third of their life, then discards a third of the cards in their hand, then sacrifices a third of the creatures they control of their choice, then sacrifices a third of the lands they control of their choice. Round up each time.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn third_up(n: u32) -> u32 {
    (n + 2) / 3
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut seq: Vec<Effect> = Vec::new();
    for p in script::all_players(state) {
        let life = script::life(state, p).max(0) as u32;
        seq.push(Effect::LoseLife { player: p, amount: third_up(life) });
    }
    for p in script::all_players(state) {
        let hand = script::hand_size(state, p);
        seq.push(Effect::Discard {
            player: p,
            count: third_up(hand),
            choice: DiscardChoice::ControllerChooses,
        });
    }
    for p in script::all_players(state) {
        let creatures = script::count_matching(
            state,
            &ObjectFilter::creature(),
            p,
        );
        seq.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: third_up(creatures),
        });
    }
    for p in script::all_players(state) {
        let lands = script::count_matching(
            state,
            &ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            p,
        );
        seq.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            count: third_up(lands),
        });
    }
    vec![Effect::Sequence(seq)]
}
