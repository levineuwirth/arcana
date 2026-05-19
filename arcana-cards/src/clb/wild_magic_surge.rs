//! Wild Magic Surge — `{R}{R}` instant, "Destroy target permanent an opponent controls.
//! Its controller reveals cards from the top of their library until they reveal a permanent
//! card that shares a card type with the destroyed permanent. That player puts that card onto
//! the battlefield and shuffles the rest into their library."
//!
//! GAP: Reveal-until-matching mechanic (reveal from library until finding a permanent of
//! matching type, put it onto battlefield, shuffle rest).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wild Magic Surge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target permanent an opponent controls. Its controller reveals cards from the top of their library until they reveal a permanent card that shares a card type with the destroyed permanent. That player puts that card onto the battlefield and shuffles the rest into their library.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: reveal-until mechanic (reveal from library until matching permanent type found, put onto battlefield)
    vec![Effect::DestroyPermanent { target: *id }]
}
