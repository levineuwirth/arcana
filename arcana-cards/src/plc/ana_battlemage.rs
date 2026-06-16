//! Ana Battlemage — `{2}{G}` 2/2 Human Wizard.
//! Kicker {2}{U} and/or {1}{B}. (GAP — Kicker is not an expressible keyword
//! and the "if it was kicked with its … kicker" intervening-if has no
//! condition helper; the two ETB triggers are emitted unconditionally.)
//! ETB (U kicker): target player discards three cards.
//! ETB (B kicker): tap target untapped creature and that creature deals
//! damage equal to its power to its controller.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ana Battlemage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker keyword + kicker payment not expressible.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if it was kicked with its {2}{U} kicker"
            // has no condition helper; firing unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            // GAP: intervening-if "if it was kicked with its {1}{B} kicker".
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tap_and_self_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().untapped_only(),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_discard_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard {
        player: *p,
        count: 3,
        choice: DiscardChoice::ControllerChooses,
    }]
}

fn etb_tap_and_self_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let pow = script::power_of(state, *id).max(0) as u32;
    let controller = script::target_controller(state, *id, trig.controller);
    vec![
        Effect::Tap { target: *id },
        Effect::DealDamage {
            source: *id,
            target: DamageTarget::Player(controller),
            amount: pow,
        },
    ]
}
