//! Unexpected Allies — `{1}{R}` sorcery. "Target nontoken creature you
//! control gets +2/+0 and gains double team until end of turn. It also
//! gains first strike until end of turn if it has the same name as
//! another creature you control or a creature card in your graveyard."
//!
//! # GAP
//! - "Double team" keyword is not in the supported keyword surface.
//! - Conditional first strike (same-name check across battlefield and
//!   graveyard) is inexpressible without state access in the resolver.
//! The +2/+0 pump is modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unexpected Allies");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target nontoken creature you control gets +2/+0 and gains double team until end of turn. It also gains first strike until end of turn if it has the same name as another creature you control or a creature card in your graveyard.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: "double team" keyword not in supported surface
    // GAP: conditional first strike based on same-name check not expressible
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
