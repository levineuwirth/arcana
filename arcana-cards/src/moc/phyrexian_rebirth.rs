//! Phyrexian Rebirth — `{4}{W}{W}` sorcery. "Destroy all creatures, then
//! create an X/X colorless Phyrexian Horror artifact creature token, where X
//! is the number of creatures destroyed this way."
//! GAP: counting how many creatures were actually destroyed by the wipe (to set
//! the token's P/T) requires post-effect state tracking not available via script.
//! Board-wipe is implemented; token creation is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Rebirth");
    let _phyrexian = reg.interner_mut().intern("Phyrexian");
    let _horror = reg.interner_mut().intern("Horror");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures, then create an X/X colorless Phyrexian Horror artifact creature token, where X is the number of creatures destroyed this way.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // GAP: X/X token where X = number of creatures destroyed (post-effect count not accessible)
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
