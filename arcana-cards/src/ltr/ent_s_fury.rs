//! Ent's Fury — `{1}{G}` sorcery. "Put a +1/+1 counter on target
//! creature you control if its power is 4 or greater. Then that creature
//! gets +1/+1 until end of turn and fights target creature you don't
//! control."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ent's Fury");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a +1/+1 counter on target creature you control if its \
                   power is 4 or greater. Then that creature gets +1/+1 until \
                   end of turn and fights target creature you don't control."
                .into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(
                            arcana_core::targets::ControllerConstraint::You,
                        ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(
                            arcana_core::targets::ControllerConstraint::Opponent,
                        ),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(mine)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(theirs)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    let mut effects = Vec::new();
    if script::power_of(state, *mine) >= 4 {
        effects.push(Effect::AddCounters {
            target: *mine,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects.push(Effect::Pump {
        target: *mine,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    });
    effects.push(Effect::Fight { a: *mine, b: *theirs });
    effects
}
