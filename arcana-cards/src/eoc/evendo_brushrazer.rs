//! Evendo Brushrazer — `{2}{R}` 2/2 Insect Warrior.
//!
//! Whenever you sacrifice a nontoken permanent, exile the top card of your
//! library.
//! During your turn, as long as you've sacrificed a nontoken permanent this
//! turn, you may play cards exiled with this creature.
//! {T}, Sacrifice a land: Add {R}{R}.
//!
//! The sacrifice trigger uses `Effect::ImpulseExile` (exile the top card with a
//! play-permission window), which folds in the second static line's "you may
//! play cards exiled with this creature" intent. The standalone conditional-
//! play static itself is GAP'd (no static play-permission primitive).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evendo Brushrazer");
    let insect = reg.interner_mut().intern("Insect");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: standalone "during your turn, as long as you've sacrificed a nontoken
    // permanent this turn, you may play cards exiled with this creature" static —
    // the play-permission is approximated by ImpulseExile on the trigger.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                },
                intervening_if: None,
                effect: impulse_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a land: Add {R}{R}.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::LAND.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_rr,
            }),
    )
}

fn impulse_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}

fn add_rr(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source); 2],
    }]
}
