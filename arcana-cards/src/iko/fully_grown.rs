//! Fully Grown — `{2}{G}` instant. "Target creature gets +3/+3 until
//! end of turn. Put a trample counter on it."
//!
//! The +3/+3 is expressed. The trample counter is wired as
//! `CounterKind::Named("trample")` plus a permanent Trample grant
//! (CR 122.1g) — see the narrowed note in the resolver.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fully Grown");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("trample");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +3/+3 until end of turn. Put a trample counter on it.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(kind) = reg.interner().lookup("trample").map(CounterKind::Named) {
        // Trample counter + the keyword it grants (CR 122.1g). Modeled as a
        // permanent grant; narrowed GAP: removing the counter later would
        // not revoke the keyword.
        effects.push(Effect::AddCounters { target: *id, kind, count: 1 });
        effects.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Trample,
            duration: Duration::Permanent,
        });
    }
    effects
}
