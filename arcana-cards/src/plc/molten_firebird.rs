//! Molten Firebird — `{4}{R}` Creature — Phoenix, 2/2.
//!
//! Oracle:
//! * Flying.
//! * When this creature dies, return it to the battlefield under its owner's
//!   control at the beginning of the next end step and you skip your next draw
//!   step.
//! * `{4}{R}: Exile this creature.`
//!
//! The dies trigger is emitted, but its effect is GAP'd: the available
//! `DelayedAction` returns are `ReturnToHand` / `ReturnFromExileToBattlefield`,
//! neither of which returns a card from the graveyard to the battlefield, and
//! "skip your next draw step" has no Effect/DelayedAction primitive. The
//! `{4}{R}: Exile this creature` activated ability is fully expressed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Molten Firebird");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{R}: Exile this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_self,
            }),
    )
}

fn on_dies(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "return it to the battlefield under its owner's control at the
    // beginning of the next end step and you skip your next draw step." No
    // DelayedAction returns a card from the graveyard to the battlefield
    // (only ReturnToHand / ReturnFromExileToBattlefield exist), and there is
    // no primitive for "skip your next draw step".
    Vec::new()
}

fn exile_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ExilePermanent { target: ctx.source }]
}
