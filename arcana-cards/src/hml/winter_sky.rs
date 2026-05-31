//! Winter Sky — `{R}` sorcery. "Flip a coin. If you win the flip,
//! Winter Sky deals 1 damage to each creature and each player. If you
//! lose the flip, each player draws a card."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Winter Sky");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Flip a coin. If you win the flip, Winter Sky deals 1 damage to each creature and each player. If you lose the flip, each player draws a card."
                .into(),
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
    // Win: 1 damage to each creature and each player.
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut win: Vec<Effect> = Vec::new();
    win.push(Effect::ForEach {
        targets: creature_ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 1,
        }),
    });
    for p in script::all_players(state) {
        win.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 1,
        });
    }

    // Lose: each player draws a card.
    let lose: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect();

    vec![Effect::FlipCoin {
        player: entry.controller,
        win: Box::new(Effect::Sequence(win)),
        lose: Some(Box::new(Effect::Sequence(lose))),
    }]
}
