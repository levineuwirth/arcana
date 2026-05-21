//! Gnawing Crescendo — `{2}{R}` instant. "Creatures you control get
//! +2/+0 until end of turn. Whenever a nontoken creature you control
//! dies this turn, create a 1/1 black Rat creature token with 'This
//! token can't block.'" The dies-during-turn delayed trigger has no
//! catalog primitive (DelayedAction is keyed off a single known id).
//! We pump your creatures and GAP the delayed token spawn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnawing Crescendo");
    let _rat = reg.interner_mut().intern("Rat");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control get +2/+0 until end of turn. Whenever a nontoken creature you control dies this turn, create a 1/1 black Rat creature token with \"This token can't block.\"".into(),
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
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::Pump {
            target: id,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    // GAP: 'whenever a nontoken creature you control dies this turn,
    // create a 1/1 black Rat (can't block) token' — delayed
    // any-source trigger with token spawn isn't expressible
    // (DelayedAction is keyed off a single known id).
    effects
}
