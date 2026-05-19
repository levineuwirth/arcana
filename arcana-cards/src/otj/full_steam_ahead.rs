//! Full Steam Ahead — `{3}{G}{G}` sorcery.
//! "Until end of turn, each creature you control gets +2/+2 and gains trample and
//! 'This creature can't be blocked by more than one creature.'"
//! The "can't be blocked by more than one creature" ability is not expressible via GrantKeyword.
//! GAP: no Effect for granting "can't be blocked by more than one creature" to a set of permanents.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Full Steam Ahead");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, each creature you control gets +2/+2 and gains trample and \"This creature can't be blocked by more than one creature.\"".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature();
    let ids = script::ids_matching(state, &filter, entry.controller);
    let mut effects = vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 2,
                toughness: 2,
                duration: Duration::EndOfTurn,
                keywords: vec![KeywordAbility::Trample],
            }),
        },
    ];
    // GAP: no Effect for granting "can't be blocked by more than one creature"
    let _ = ids;
    effects
}
