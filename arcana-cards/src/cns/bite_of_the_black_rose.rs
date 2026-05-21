//! Bite of the Black Rose — `{3}{B}` sorcery. "Will of the council —
//! Starting with you, each player votes for sickness or psychosis. If
//! sickness gets more votes, creatures your opponents control get
//! -2/-2 until end of turn. If psychosis gets more votes or the vote
//! is tied, each opponent discards two cards." Voting has no engine
//! primitive — best-effort: apply BOTH effects (the union) since we
//! can't branch on the vote.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bite of the Black Rose");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: 'Will of the council' voting mechanic — no engine primitive; we apply both branches.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Will of the council — Starting with you, each player votes for sickness or psychosis. If sickness gets more votes, creatures your opponents control get -2/-2 until end of turn. If psychosis gets more votes or the vote is tied, each opponent discards two cards.".into(),
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
    let mut effects = vec![Effect::ForEach {
        targets: script::ids_matching(
            state,
            &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            entry.controller,
        ),
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -2,
            toughness: -2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }];
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::Discard {
            player: opp,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects
}
