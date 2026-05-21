//! Anzrag's Rampage — `{3}{R}{R}` sorcery. "Destroy all artifacts you
//! don't control, then exile the top X cards of your library, where X
//! is the number of artifacts that were put into graveyards from the
//! battlefield this turn. You may put a creature card exiled this way
//! onto the battlefield. It gains haste. Return it to your hand at the
//! beginning of the next end step." The 'artifacts-died-this-turn'
//! count + exile-top-X-with-conditional-play isn't catalog-shaped;
//! emit the opponent-artifact wipe, GAP the rest.

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
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    let effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect();
    // GAP: 'artifacts put into graveyards from the battlefield this
    // turn' tally; exile-top-X-with-conditional-creature-play +
    // delayed return-to-hand at next end step.
    effects
}
