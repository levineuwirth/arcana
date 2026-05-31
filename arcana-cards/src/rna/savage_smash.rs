//! Savage Smash — `{1}{R}{G}` sorcery. "Target creature you control
//! gets +2/+2 until end of turn. It fights target creature you don't
//! control." A pump-then-fight spell with two targets.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Savage Smash");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gets +2/+2 until end of turn. \
                   It fights target creature you don't control."
                .into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
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
    let mut it = entry.targets.targets.iter();
    let Some(TargetChoice::Object(mine)) = it.next() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(theirs)) = it.next() else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *mine,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Fight { a: *mine, b: *theirs },
    ]
}
