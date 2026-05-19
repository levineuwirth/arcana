//! Trap Essence — `{G}{U}{R}` instant. "Counter target creature spell. Put two
//! +1/+1 counters on up to one target creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trap Essence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target creature spell. Put two +1/+1 counters on up to one target creature.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Spell(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
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

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut iter = entry.targets.targets.iter();
    let Some(t0) = iter.next() else { return Vec::new(); };
    let TargetChoice::Object(spell_id) = t0 else { return Vec::new(); };
    let mut effects = vec![Effect::Counter { target: *spell_id }];
    if let Some(TargetChoice::Object(creature_id)) = iter.next() {
        effects.push(Effect::AddCounters {
            target: *creature_id,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        });
    }
    effects
}
