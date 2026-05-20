//! Conduct Electricity — `{4}{R}` instant. "Conduct Electricity deals
//! 6 damage to target creature and 2 damage to up to one target
//! creature token."
//!
//! Two creature targets: 6 damage to the first, 2 damage to the
//! second (the "up to one" token target — token restriction not
//! modeled as a filter).
//!
//! GAP: the second target's "creature token" restriction is not
//! expressible as a target filter (unfiltered second creature target).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conduct Electricity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Conduct Electricity deals 6 damage to target creature and 2 damage to up to one target creature token.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    let mut it = entry.targets.targets.iter();
    if let Some(TargetChoice::Object(id)) = it.next() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 6,
        });
    }
    if let Some(TargetChoice::Object(id)) = it.next() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 2,
        });
    }
    effects
}
