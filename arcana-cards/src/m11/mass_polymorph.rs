//! Mass Polymorph — `{5}{U}` sorcery. "Exile all creatures you control,
//! then reveal cards from the top of your library until you reveal that
//! many creature cards. Put all creature cards revealed this way onto
//! the battlefield, then shuffle the rest of the revealed cards into
//! your library."
//!
//! The exile-all-your-creatures step is expressible as a `ForEach`
//! `ExilePermanent` over your creatures. The reveal-until step is
//! count-driven (reveal until N creature cards are found, where N is the
//! number exiled, then put ALL of them onto the battlefield) — the
//! `RevealUntil` primitive only finds a SINGLE card and the count
//! coupling to the just-exiled creatures is not expressible, so that
//! half is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mass Polymorph");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all creatures you control, then reveal cards from the top of your library until you reveal that many creature cards. Put all creature cards revealed this way onto the battlefield, then shuffle the rest of the revealed cards into your library.".into(),
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
    // Exile all creatures you control.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
    // GAP: reveal cards until you reveal as many creature cards as were
    // exiled, put all of them onto the battlefield, shuffle the rest —
    // RevealUntil finds only a single card and the count of revealed
    // creatures cannot be coupled to the just-exiled creatures.
}
