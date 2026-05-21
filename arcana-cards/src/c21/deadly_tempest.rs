//! Deadly Tempest — `{4}{B}{B}` sorcery. "Destroy all creatures. Each
//! player loses life equal to the number of creatures they controlled
//! that were destroyed this way."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deadly Tempest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures. Each player loses life equal to the number of creatures they controlled that were destroyed this way.".into(),
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
    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    );
    let mut out: Vec<Effect> = creature_ids
        .iter()
        .map(|id| Effect::DestroyPermanent { target: *id })
        .collect();
    for p in script::all_players(state) {
        let n = script::count_matching(
            state,
            &ObjectFilter::creature()
                .controlled_by(arcana_core::targets::ControllerConstraint::You),
            p,
        );
        if n > 0 {
            out.push(Effect::LoseLife { player: p, amount: n });
        }
    }
    out
}
