//! Second Harvest — `{2}{G}{G}` instant. "For each token you control,
//! create a token that's a copy of that permanent." Enumerates the
//! token permanents you control at resolution and emits one
//! [`Effect::CopyPermanent`] per token id (one CopyPermanent for each,
//! so the per-id target is stamped in directly rather than relying on
//! ForEach, which does not retarget CopyPermanent).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Second Harvest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "For each token you control, create a token that's a copy of that permanent."
                .into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .tokens_only(),
        entry.controller,
    );
    ids.into_iter()
        .map(|id| Effect::CopyPermanent { target: id })
        .collect()
}
