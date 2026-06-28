//! Celebrate the Harvest — `{3}{G}` sorcery. "Search your library for up to X
//! basic land cards, where X is the number of different powers among creatures
//! you control. Put those cards onto the battlefield tapped, then shuffle."
//! Dynamic X: count distinct power values among your creatures via
//! ids_matching + power_of per creature + HashSet dedup.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Celebrate the Harvest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to X basic land cards, where X is the number of different powers among creatures you control. Put those cards onto the battlefield tapped, then shuffle.".into(),
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
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let distinct_powers: std::collections::HashSet<i32> = creature_ids
        .iter()
        .map(|&id| script::power_of(state, id))
        .collect();
    let x = distinct_powers.len() as u32;
    let land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    (0..x)
        .map(|_| Effect::TutorToBattlefield {
            player: entry.controller,
            filter: land_filter.clone(),
            tapped: true,
        })
        .collect()
}
