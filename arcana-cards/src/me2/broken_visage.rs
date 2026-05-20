//! Broken Visage — `{4}{B}` instant. "Destroy target nonartifact
//! attacking creature. It can't be regenerated. Create a black Spirit
//! creature token. Its power is equal to that creature's power and
//! its toughness is equal to that creature's toughness. Sacrifice the
//! token at the beginning of the next end step."
//!
//! Destroy + a Spirit token whose P/T is snapshotted from the
//! destroyed creature. The "nonartifact attacking" target restriction
//! and the delayed token-sacrifice (token id unavailable) are GAPs.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Broken Visage");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target nonartifact attacking creature. It can't be regenerated. Create a black Spirit creature token. Its power is equal to that creature's power and its toughness is equal to that creature's toughness. Sacrifice the token at the beginning of the next end step.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let pwr = script::power_of(state, *id);
    let tuf = script::toughness_of(state, *id);
    let spirit = reg.interner().lookup("Spirit").expect("interned");
    let mut sub = SubtypeSet::default();
    sub.0.insert(spirit);
    // GAP: delayed sacrifice of the created token — token id unavailable.
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: spirit,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: sub,
                power: Some(PtValue::Fixed(pwr.max(0))),
                toughness: Some(PtValue::Fixed(tuf.max(0))),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
