//! Anzrag's Rampage — `{3}{R}{R}` sorcery. "Destroy all artifacts you
//! don't control, then exile the top X cards of your library, where X
//! is the number of artifacts that were put into graveyards from the
//! battlefield this turn. You may put a creature card exiled this way
//! onto the battlefield. It gains haste. Return it to your hand at the
//! beginning of the next end step."
//!
//! # GAP: Count artifacts that died this turn (turn-history tracking
//! not in script helpers); exile top X of library; may put creature
//! from exile to battlefield with haste; return-to-hand delayed
//! trigger at next end step. All riders after the destroy are GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ControllerConstraint};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all artifacts you don't control, then exile the top X cards of your library, where X is the number of artifacts that were put into graveyards from the battlefield this turn. You may put a creature card exiled this way onto the battlefield. It gains haste. Return it to your hand at the beginning of the next end step.".into(),
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
    let artifact_filter = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &artifact_filter, entry.controller);
    let destroy: Vec<Effect> = ids.into_iter().map(|id| Effect::DestroyPermanent { target: id }).collect();
    // GAP: count artifacts that died this turn (turn-history not in script);
    // exile top X of library; optional creature-to-battlefield with haste;
    // delayed return-to-hand trigger at next end step
    destroy
}
