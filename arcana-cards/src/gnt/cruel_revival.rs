//! Cruel Revival — `{4}{B}` instant. "Destroy target non-Zombie
//! creature. It can't be regenerated. Return up to one target Zombie
//! card from your graveyard to your hand."
//!
//! The non-Zombie target restriction is expressed with
//! `TargetFilter::Permanent(ObjectFilter::creature().without_subtype_sym(..))`.
//! Note: "It can't be regenerated" and a separate "up to one" Zombie
//! return target are not directly expressible — we destroy the
//! primary target and document the missing pieces as GAPs.

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
    let name = reg.interner_mut().intern("Cruel Revival");
    let zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target non-Zombie creature. It can't be regenerated. Return up to one target Zombie card from your graveyard to your hand.".into(),
            target_requirements: vec![TargetRequirement {
                // Target non-Zombie creature.
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().without_subtype_sym(zombie),
                ),
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
    // GAP: "can't be regenerated" rider; optional second "Zombie card
    // from graveyard" return target.
    vec![Effect::DestroyPermanent { target: *id }]
}
