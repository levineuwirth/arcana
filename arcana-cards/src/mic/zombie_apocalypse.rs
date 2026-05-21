//! Zombie Apocalypse — `{3}{B}{B}{B}` sorcery. "Return all Zombie
//! creature cards from your graveyard to the battlefield tapped,
//! then destroy all Humans."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zombie Apocalypse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return all Zombie creature cards from your graveyard to the battlefield tapped, then destroy all Humans.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Destroy all Humans" — enumerate Human creatures on the
    // battlefield via the subtype filter and destroy each.
    let humans =
        script::ids_matching(state, &script::subtype_filter(reg, "Human"), entry.controller);
    vec![Effect::ForEach {
        targets: humans,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
    // GAP: "return all Zombie creature cards from your graveyard to
    // the battlefield tapped" — there is no batch graveyard-to-
    // battlefield primitive (the return effects need a pre-resolved
    // target id and ids_matching only scans the battlefield), so the
    // mass reanimation is omitted.
}
