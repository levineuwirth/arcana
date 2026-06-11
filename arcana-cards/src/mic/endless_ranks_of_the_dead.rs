//! Endless Ranks of the Dead — `{2}{B}{B}` enchantment.
//! "At the beginning of your upkeep, create X 2/2 black Zombie creature
//! tokens, where X is half the number of Zombies you control, rounded
//! down."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Endless Ranks of the Dead");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: spawn_zombies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create X 2/2 black Zombie creature tokens, where X is half the
/// number of Zombies you control, rounded down."
fn spawn_zombies(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombies_you_control = script::count_matching(
        state,
        &script::subtype_filter(reg, "Zombie").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let x = zombies_you_control / 2;
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    (0..x)
        .map(|_| {
            let mut subtypes = SubtypeSet::default();
            subtypes.0.insert(zombie);
            Effect::CreateToken {
                controller: trig.controller,
                token: TokenDefinition {
                    name: zombie,
                    colors: ColorSet::black(),
                    types: TypeLine::CREATURE.into(),
                    subtypes,
                    power: Some(PtValue::Fixed(2)),
                    toughness: Some(PtValue::Fixed(2)),
                    keywords: vec![],
                    abilities: vec![],
                },
            }
        })
        .collect()
}
