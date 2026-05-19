//! Reckless Ransacking — `{1}{R}` instant, "Target creature gets +3/+2 until
//! end of turn. Create a Treasure token. (It's an artifact with '{T},
//! Sacrifice this artifact: Add one mana of any color.')"
//!
//! # GAP: Treasure token has an activated tap ability; TokenDefinition in
//! engine API does not support activated abilities on tokens

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Ransacking");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +3/+2 until end of turn. Create a Treasure token.".into(),
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
            power: 3,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        // GAP: Treasure token has an activated ability; TokenDefinition does not support activated abilities
    ]
}
