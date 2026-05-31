//! Become Brutes — `{1}{R}` sorcery. "One or two target creatures each
//! gain haste until end of turn. For each of those creatures, create a
//! Monster Role token attached to it."
//!
//! The haste grant on one or two target creatures is expressible
//! (`GrantKeyword` Haste per target). The Monster Role token (an Aura-like
//! Role enchantment that attaches to the creature, replaces any other Role,
//! and grants +1/+1 and trample) has no catalog primitive — there is no
//! Role-token / attach-aura effect — so that rider is an honest GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Become Brutes");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "One or two target creatures each gain haste until end of turn. For each of those creatures, create a Monster Role token attached to it.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(2),
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
    // GAP: no Monster Role token creation / aura-attach primitive; only the
    // haste grant on each target creature is expressed.
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::Haste,
                duration: Duration::EndOfTurn,
            });
        }
    }
    effects
}
