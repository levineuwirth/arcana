//! Selfcraft Mechan — `{3}{U}` 3/4 blue Artifact Creature — Robot Artificer.
//! "When this creature enters, you may sacrifice an artifact. When you do,
//! put a +1/+1 counter on target creature and draw a card."
//!
//! # GAP: "you may sacrifice an artifact" triggers a secondary triggered ability ("when you do");
//! the engine has no optional sacrifice-then-trigger-secondary model.
//! The ETB trigger is modeled; the sacrifice cost gate and counter-plus-draw chain are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Selfcraft Mechan");
    let robot = reg.interner_mut().intern("Robot");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                // GAP: "you may sacrifice an artifact" cost gate is not modeled.
                // GAP: secondary "when you do" trigger chain is not modeled.
                effect: etb_effect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn etb_effect(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional sacrifice-an-artifact cost gate not modeled.
    // Emit the counter + draw assuming the cost was paid, as best-effort.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}
