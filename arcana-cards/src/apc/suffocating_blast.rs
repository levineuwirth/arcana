//! Suffocating Blast — `{1}{U}{U}{R}` instant. "Counter target spell
//! and Suffocating Blast deals 3 damage to target creature."

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
    let name = reg.interner_mut().intern("Suffocating Blast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell and Suffocating Blast deals 3 damage to target creature.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut iter = entry.targets.targets.iter();
    let Some(TargetChoice::Object(spell)) = iter.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(crea)) = iter.next() else { return Vec::new(); };
    vec![
        Effect::Counter { target: *spell },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*crea),
            amount: 3,
        },
    ]
}
