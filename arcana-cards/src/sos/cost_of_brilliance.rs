//! Cost of Brilliance — `{2}{B}` sorcery, "Target player draws two cards and loses
//! 2 life. Put a +1/+1 counter on up to one target creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cost of Brilliance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player draws two cards and loses 2 life. Put a +1/+1 counter on up to one target creature.".into(),
                target_requirements: vec![
                    TargetRequirement::target_player(),
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
    let Some(first_target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(target_player) = first_target else { return Vec::new(); };

    let mut effects = vec![
        Effect::DrawCards { player: *target_player, count: 2 },
        Effect::LoseLife { player: *target_player, amount: 2 },
    ];

    if let Some(second_target) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(creature_id) = second_target {
            effects.push(Effect::AddCounters {
                target: *creature_id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }

    effects
}
