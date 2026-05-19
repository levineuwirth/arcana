//! Kaya's Wrath — `{W}{W}{B}{B}` sorcery.
//! "Destroy all creatures. You gain life equal to the number of creatures you controlled
//! that were destroyed this way."
//! GAP: life gain should reflect only creatures you controlled that were actually destroyed;
//! approximated as pre-wipe count of your creatures.

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
    let name = reg.interner_mut().intern("Kaya's Wrath");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{B}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. You gain life equal to the number of creatures you controlled that were destroyed this way.".into(),
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
    let my_creatures = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let all_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut effects: Vec<Effect> = all_ids
        .into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect();
    effects.push(Effect::GainLife { player: entry.controller, amount: my_creatures });
    effects
}
