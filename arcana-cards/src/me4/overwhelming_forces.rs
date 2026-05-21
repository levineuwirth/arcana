//! Overwhelming Forces — `{6}{B}{B}` sorcery. "Destroy all creatures
//! target opponent controls. Draw a card for each creature destroyed
//! this way." Count opponent's creatures, then issue destroys and a
//! matching draw.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Overwhelming Forces");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures target opponent controls. Draw a card for each creature destroyed this way.".into(),
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
    // GAP: filter 'creatures controlled by target player' — we can
    // only use ControllerConstraint::Opponent (any opponent). For the
    // typical 1v1 case this matches the target opponent.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(_p) = target else { return Vec::new(); };
    let filter = ObjectFilter::creature()
        .controlled_by(arcana_core::targets::ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, entry.controller);
    let n = ids.len() as u32;
    let mut effects: Vec<Effect> = ids.into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect();
    effects.push(Effect::DrawCards { player: entry.controller, count: n });
    effects
}
