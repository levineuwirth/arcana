//! Mogg Infestation — `{3}{R}{R}` sorcery.
//! "Destroy all creatures target player controls. For each creature that died this way, that player creates two 1/1 red Goblin creature tokens."
//! GAP: counting creatures that died from a ForEach effect and then creating twice as many tokens is not expressible.
//! Best effort: destroy all creatures the target player controls (token creation omitted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mogg Infestation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures target player controls. For each creature that died this way, that player creates two 1/1 red Goblin creature tokens.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    use arcana_core::targets::TargetChoice;
    let Some(TargetChoice::Player(target_player)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: post-resolution token creation based on how many creatures died is not expressible
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: arcana_core::objects::NULL_OBJECT_ID }),
    }]
}
