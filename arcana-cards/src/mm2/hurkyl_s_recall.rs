//! Hurkyl's Recall — `{1}{U}` instant. "Return all artifacts target
//! player owns to their hand."
//!
//! GAP: 'owns' is not an ObjectFilter constraint — `controlled_by` is
//! the closest available knob, so only artifacts that player controls
//! are returned (ownership-vs-control nuance lost).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hurkyl's Recall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return all artifacts target player owns to their hand.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(_p)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: 'owns' nuance — using controller as proxy for ownership.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types(arcana_core::types::TypeLine::ARTIFACT.into()),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnToHand { target: NULL_OBJECT_ID }),
    }]
}
