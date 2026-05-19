//! Mabel's Mettle — `{1}{W}` instant, "Target creature gets +2/+2 until
//! end of turn. Up to one other target creature gets +1/+1 until end of
//! turn."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mabel's Mettle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +2/+2 until end of turn. Up to one other target creature gets +1/+1 until end of turn.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
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
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id_a) = first else { return Vec::new(); };
    let mut effects = vec![Effect::Pump {
        target: *id_a,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(second) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(id_b) = second {
            effects.push(Effect::Pump {
                target: *id_b,
                power: 1,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            });
        }
    }
    effects
}
