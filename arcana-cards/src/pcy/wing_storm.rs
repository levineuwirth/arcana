//! Wing Storm — `{2}{G}` sorcery. "Wing Storm deals damage to each
//! player equal to twice the number of creatures that player controls
//! with flying."
//!
//! Implementation: iterate all_players; for each player count their
//! flying creatures via script::count_matching with that player as the
//! controller reference; deal 2 × count damage to each player.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wing Storm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Wing Storm deals damage to each player equal to twice the number of creatures that player controls with flying.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let per_player: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| {
            let n = script::count_matching(
                state,
                &ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_keyword(KeywordAbility::Flying),
                p,
            );
            Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Player(p),
                amount: 2 * n,
            }
        })
        .collect();
    vec![Effect::Sequence(per_player)]
}
