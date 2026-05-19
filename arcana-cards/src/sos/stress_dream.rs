//! Stress Dream — `{3}{U}{R}` instant. "Stress Dream deals 5 damage
//! to up to one target creature. Look at the top two cards of your
//! library. Put one into your hand and the other on the bottom of
//! your library."
//!
//! # GAP: look at top 2 and selective draw/bottom (no "look at top N, choose 1 to hand" effect)
//! DealDamage to up to one creature is expressible. The look-and-select
//! from top 2 is not in the catalog. Emitting damage only.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stress Dream");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Stress Dream deals 5 damage to up to one target creature. Look at the top two cards of your library. Put one into your hand and the other on the bottom of your library.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
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
    let mut effects = Vec::new();
    if let Some(target) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*id),
                amount: 5,
            });
        }
    }
    // GAP: look at top 2 cards, put one to hand and one to bottom of library
    effects
}
