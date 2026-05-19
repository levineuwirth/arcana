//! Urge to Feed — `{B}{B}` instant, "Target creature gets -3/-3 until end of
//! turn. You may tap any number of untapped Vampires you control. If you do,
//! put a +1/+1 counter on each of those Vampires."
//!
//! GAP: optional tap-untapped-Vampires-for-counters mechanic not expressible.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urge to Feed");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets -3/-3 until end of turn. You may tap any number of untapped Vampires you control. If you do, put a +1/+1 counter on each of those Vampires.".into(),
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
    vec![
        Effect::Pump {
            target: *id,
            power: -3,
            toughness: -3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        // GAP: optional tap untapped Vampires for +1/+1 counters mechanic
    ]
}
