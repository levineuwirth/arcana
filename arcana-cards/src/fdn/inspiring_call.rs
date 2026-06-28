//! Inspiring Call — `{2}{G}` instant. "Draw a card for each creature you control with a +1/+1 counter on it. Those creatures gain indestructible until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inspiring Call");
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
                text: "Draw a card for each creature you control with a +1/+1 counter on it. Those creatures gain indestructible until end of turn.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter {
        has_counter: Some(CounterKind::PlusOnePlusOne),
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
    };
    let count = script::count_matching(state, &filter, entry.controller);
    vec![
        Effect::DrawCards { player: entry.controller, count },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                entry.source,
                filter,
                KeywordAbility::Indestructible,
                Duration::EndOfTurn,
            ),
        },
    ]
}
