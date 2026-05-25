//! Ubul Sar Gatekeepers — `{3}{B}` 2/4 black Creature — Zombie Soldier.
//! "When this creature enters, if you control two or more Gates, target
//! creature an opponent controls gets -2/-2 until end of turn."
//! GAP: intervening-if "if you control two or more Gates" not expressible.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ubul Sar Gatekeepers");
    let zombie = reg.interner_mut().intern("Zombie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening-if "if you control two or more Gates" not expressible
                intervening_if: None,
                effect: etb_weaken_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Creature,
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn etb_weaken_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: -2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
