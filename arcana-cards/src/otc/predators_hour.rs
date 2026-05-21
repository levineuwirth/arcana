//! Predators' Hour — `{1}{B}` sorcery. "Until end of turn, creatures
//! you control gain menace and 'Whenever this creature deals combat
//! damage to a player, exile the top card of that player's library
//! face down. You may look at and play that card for as long as it
//! remains exiled, and you may spend mana as though it were mana of
//! any color to cast that spell.'" Granting menace via ForEach +
//! GrantKeyword is expressible; the cast-from-exile rider isn't. We
//! grant menace and GAP the impulse-draw trigger.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Predators' Hour");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, creatures you control gain menace and \"Whenever this creature deals combat damage to a player, exile the top card of that player's library face down. You may look at and play that card for as long as it remains exiled, and you may spend mana as though it were mana of any color to cast that spell.\"".into(),
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
    // GAP: granted 'deals combat damage → impulse-cast from exile'
    // trigger is not in the catalog.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Menace,
            duration: Duration::EndOfTurn,
        }),
    }]
}
