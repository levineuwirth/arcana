//! Boundless Realms — `{6}{G}` sorcery. "Search your library for up
//! to X basic land cards, where X is the number of lands you control,
//! put them onto the battlefield tapped, then shuffle." X is the
//! count of lands you control; TutorToBattlefield is a single-card
//! primitive, so we repeat it X times. We can't constrain to 'basic'
//! supertype (no helper), so the filter is LAND.

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
    let name = reg.interner_mut().intern("Boundless Realms");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to X basic land cards, where X is the number of lands you control, put them onto the battlefield tapped, then shuffle.".into(),
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
    let x = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    // GAP: cannot constrain TutorToBattlefield to the 'basic' supertype
    // — filter is LAND.
    let mut effects = Vec::with_capacity(x as usize);
    for _ in 0..x {
        effects.push(Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        });
    }
    effects
}
