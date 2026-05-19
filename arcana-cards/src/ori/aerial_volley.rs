//! Aerial Volley — `{G}` instant. "Aerial Volley deals 3 damage
//! divided as you choose among one, two, or three target creatures
//! with flying."
//!
//! # GAP
//! "Divided as you choose" (split damage) requires player input at
//! resolution to assign amounts; no Effect variant for divided damage.
//! Targeting creatures restricted to flying is also not supported by
//! `TargetFilter::Creature`. Best effort: deal 1 damage to each of up
//! to three creature targets with no flying filter and no split.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aerial Volley");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Aerial Volley deals 3 damage divided as you choose among one, two, or three target creatures with flying.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Creature,
                    count: TargetCount::UpTo(3),
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
    // GAP: divided damage (player chooses amounts totalling 3) not expressible; flying-only filter not supported
    // Best effort: 1 damage to each targeted creature
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::DealDamage {
                    source: entry.source,
                    target: DamageTarget::Object(*id),
                    amount: 1,
                })
            } else {
                None
            }
        })
        .collect()
}
