//! Schismotivate — `{1}{U}{R}` instant. "Target creature gets +4/+0
//! until end of turn. Another target creature gets -4/-0 until end of
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
    let name = reg.interner_mut().intern("Schismotivate");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +4/+0 until end of turn. Another target creature gets -4/-0 until end of turn.".into(),
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
    let mut it = entry.targets.targets.iter();
    let (Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) =
        (it.next(), it.next())
    else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *a,
            power: 4,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Pump {
            target: *b,
            power: -4,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
