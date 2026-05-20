//! Consume Strength — `{1}{B}{G}` instant. "Target creature gets +2/+2
//! until end of turn. Another target creature gets -2/-2 until end of
//! turn."

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
    let name = reg.interner_mut().intern("Consume Strength");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +2/+2 until end of turn. Another target creature gets -2/-2 until end of turn.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(a)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(b)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *a,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Pump {
            target: *b,
            power: -2,
            toughness: -2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
