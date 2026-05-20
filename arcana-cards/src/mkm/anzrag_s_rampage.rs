//! Anzrag's Rampage — `{3}{R}{R}` sorcery. "Destroy all artifacts you
//! don't control, then exile the top X cards of your library, where X
//! is the number of artifacts that were put into graveyards from the
//! battlefield this turn. You may put a creature card exiled this way
//! onto the battlefield. It gains haste. Return it to your hand at
//! the beginning of the next end step."
//!
//! The X-count over "artifacts that went to a graveyard this turn"
//! is not in the script::* helper surface; exile-top-X and the
//! conditional reanimate/return-to-hand-at-end-step coupling are not
//! in catalog. Models only the opponent-artifacts wipe.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anzrag's Rampage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all artifacts you don't control, then exile the top X cards of your library, where X is the number of artifacts that were put into graveyards from the battlefield this turn. You may put a creature card exiled this way onto the battlefield. It gains haste. Return it to your hand at the beginning of the next end step.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "artifacts put into graveyards this turn" count, exile-top-X library, and
    // the may-cast-creature + return-at-EOT chain are not in catalog/script surface.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
