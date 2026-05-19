//! Begin Anew — `{G}{G}{W}{W}` sorcery.
//! "Destroy all creatures. Creature cards in your hand perpetually get +1/+1."
//!
//! GAP: "creature cards in your hand perpetually get +1/+1" — no PerpetualEffect or
//! GrantPumpToHandCards variant in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Begin Anew");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. Creature cards in your hand perpetually get +1/+1.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut effects: Vec<Effect> = ids.into_iter().map(|id| Effect::DestroyPermanent { target: id }).collect();
    // GAP: "creature cards in hand perpetually get +1/+1" — no PerpetualEffect in catalog.
    effects
}
