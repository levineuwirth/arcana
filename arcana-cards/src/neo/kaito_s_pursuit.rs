//! Kaito's Pursuit — `{2}{B}` sorcery. "Target player discards two
//! cards. Ninjas and Rogues you control gain menace until end of
//! turn."

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::layers::Duration;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaito's Pursuit");
    let _ninja = reg.interner_mut().intern("Ninja");
    let _rogue = reg.interner_mut().intern("Rogue");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player discards two cards. Ninjas and Rogues you control gain menace until end of turn.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let mut effects = vec![Effect::Discard {
        player: *p,
        count: 2,
        choice: DiscardChoice::ControllerChooses,
    }];
    let ninjas = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Ninja").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let rogues = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Rogue").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut ids = ninjas;
    for id in rogues {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    if !ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Menace,
                duration: Duration::EndOfTurn,
            }),
        });
    }
    effects
}
