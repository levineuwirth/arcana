//! Chaos Mutation — `{3}{U}{R}` instant. "Exile any number of target
//! creatures controlled by different players. For each creature exiled
//! this way, its controller reveals cards from the top of their library
//! until they reveal a creature card, puts that card onto the
//! battlefield, then puts the rest on the bottom of their library in a
//! random order."
//!
//! Any-number-of targets (the "different players" restriction is a
//! targeting nuance the engine does not separately model). For each
//! targeted creature we exile it, then have its controller reveal from
//! the top of their library until a creature card is found, putting it
//! onto the battlefield and the rest on the bottom at random — exactly
//! `Effect::RevealUntil { found_dest: Battlefield, rest: BottomRandom }`.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaos Mutation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile any number of target creatures controlled by different players. For each creature exiled this way, its controller reveals cards from the top of their library until they reveal a creature card, puts that card onto the battlefield, then puts the rest on the bottom of their library in a random order.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Any,
                controller: None,
            }],
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
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        let TargetChoice::Object(id) = target else { continue };
        let controller = script::target_controller(state, *id, entry.controller);
        effects.push(Effect::ExilePermanent { target: *id });
        effects.push(Effect::RevealUntil {
            player: controller,
            filter: ObjectFilter::creature(),
            found_dest: RevealDest::Battlefield,
            rest: DigRest::BottomRandom,
            max_reveal: None,
        });
    }
    effects
}
