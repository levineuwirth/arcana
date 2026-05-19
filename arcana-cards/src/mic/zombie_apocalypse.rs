//! Zombie Apocalypse — `{3}{B}{B}{B}` sorcery. "Return all Zombie creature cards from your
//! graveyard to the battlefield tapped, then destroy all Humans."
//!
//! GAP: Returning multiple graveyard cards tapped (no tapped field in ReturnFromGraveyardToBattlefield);
//! tribal graveyard return (all Zombies). Using ids_matching for the Human destroy;
//! best-effort for zombie return (untapped, no mass graveyard recall).

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
    let name = reg.interner_mut().intern("Zombie Apocalypse");
    let _human = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
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
    // GAP: mass graveyard return of Zombie cards; tapped ETB not expressible
    let human_ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Human"),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: human_ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
