//! Sudden Spinnerets — `{G}` instant. "Target creature gets +1/+3
//! until end of turn. Put a reach counter on it. Untap it."
//!
//! The +1/+3 and Untap are expressed. The reach counter is wired as
//! `CounterKind::Named("reach")` plus a permanent Reach grant
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
    let name = reg.interner_mut().intern("Sudden Spinnerets");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("reach");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +1/+3 until end of turn. Put a reach counter on it. Untap it.".into(),
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
        power: 1,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(kind) = reg.interner().lookup("reach").map(CounterKind::Named) {
        // Reach counter + the keyword it grants (CR 122.1g). Modeled as a
        // permanent grant; narrowed GAP: removing the counter later would
        // not revoke the keyword.
        effects.push(Effect::AddCounters { target: *id, kind, count: 1 });
        effects.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Reach,
            duration: Duration::Permanent,
        });
    }
    effects.push(Effect::Untap { target: *id });
    effects
}
