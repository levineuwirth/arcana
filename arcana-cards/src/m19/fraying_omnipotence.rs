//! Fraying Omnipotence — `{3}{B}{B}` sorcery. "Each player loses half
//! their life, then discards half the cards in their hand, then
//! sacrifices half the creatures they control of their choice. Round
//! up each time."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fraying Omnipotence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player loses half their life, then discards half the cards in their hand, then sacrifices half the creatures they control of their choice. Round up each time.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        let life = script::life(state, p).max(0) as u32;
        let lose = life.div_ceil(2);
        let hand = script::hand_size(state, p);
        let disc = hand.div_ceil(2);
        let creatures = script::count_matching(
            state,
            &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            p,
        );
        let sac = creatures.div_ceil(2);
        effects.push(Effect::LoseLife { player: p, amount: lose });
        effects.push(Effect::Discard {
            player: p,
            count: disc,
            choice: DiscardChoice::ControllerChooses,
        });
        effects.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: sac,
        });
    }
    effects
}
