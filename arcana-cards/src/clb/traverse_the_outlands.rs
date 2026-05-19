//! Traverse the Outlands — `{4}{G}` sorcery.
//! "Search your library for up to X basic land cards, where X is the greatest
//! power among creatures you control. Put those cards onto the battlefield
//! tapped, then shuffle."

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
    let name = reg.interner_mut().intern("Traverse the Outlands");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to X basic land cards, where X is the greatest power among creatures you control. Put those cards onto the battlefield tapped, then shuffle.".into(),
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
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let max_power = ids
        .iter()
        .map(|&id| script::power_of(state, id).max(0) as u32)
        .max()
        .unwrap_or(0);
    let mut effects = Vec::new();
    for _ in 0..max_power {
        effects.push(Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        });
    }
    effects
}
