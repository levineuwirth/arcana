//! Elemental Spectacle — `{5}{G}` sorcery. "Vivid — Create a number of 5/5
//! red and green Elemental creature tokens equal to the number of colors among
//! permanents you control. Then you gain life equal to the number of creatures
//! you control."
//!
//! 'Number of colors among permanents you control' is not in the script::
//! helpers — GAP the token count. Lifegain half uses creature count and is
//! emittable. (Per MANDATORY: if scaling is dynamic and not computable, GAP the
//! whole sub-effect rather than emit a fixed-size stand-in.)

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
    let name = reg.interner_mut().intern("Elemental Spectacle");
    let _ = reg.interner_mut().intern("Elemental");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Vivid — Create a number of 5/5 red and green Elemental creature tokens equal to the number of colors among permanents you control. Then you gain life equal to the number of creatures you control.".into(),
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
    let creature_count = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    // GAP: 'number of colors among permanents you control' — color-distinct count
    // helper not in script::.
    vec![Effect::GainLife {
        player: entry.controller,
        amount: creature_count,
    }]
}
