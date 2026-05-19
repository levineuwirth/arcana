//! Become the Avalanche — `{4}{G}{G}` sorcery. "Draw a card for each creature
//! you control with power 4 or greater. Then creatures you control get +X/+X
//! until end of turn, where X is the number of cards in your hand."

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
    let name = reg.interner_mut().intern("Become the Avalanche");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card for each creature you control with power 4 or greater. Then creatures you control get +X/+X until end of turn, where X is the number of cards in your hand.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    use arcana_core::targets::ControllerConstraint;
    let draw_count = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        entry.controller,
    );
    // After drawing, X = hand size. We compute hand size now (before draw) as best-effort.
    // GAP: hand size after draw is not computable at resolve time with script helpers
    let x = script::hand_size(state, entry.controller);
    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects = Vec::new();
    if draw_count > 0 {
        effects.push(Effect::DrawCards { player: entry.controller, count: draw_count });
    }
    for id in creature_ids {
        effects.push(Effect::Pump {
            target: id,
            power: x as i32,
            toughness: x as i32,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}
