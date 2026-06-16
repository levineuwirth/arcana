//! Tangled Colony — `{1}{B}` 3/2 black Rat.
//!
//! Rules text:
//! * This creature can't block. (static — modeled as the source forbidding
//!   itself from blocking while on the battlefield)
//! * When this creature dies, create X 1/1 black Rat creature tokens with
//!   "This token can't block," where X is the amount of damage dealt to it
//!   this turn.
//!
//! The dies trigger's count (X = damage dealt to this creature this turn) has
//! no `script::*` accessor — the `SelfDies` trigger carries no damage-taken
//! amount, and there is no helper for "damage dealt to it this turn". Because
//! the token count is dynamic and uncomputable with the demonstrated API, the
//! dies effect is GAP'd rather than emitting a wrong fixed count.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tangled Colony");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "This creature can't block." — its own static can't-block.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: forbid_self_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "When this creature dies, create X 1/1 black Rats ..." — GAP'd.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_rats,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Forbid this creature from ever blocking while it is on the battlefield.
fn forbid_self_block(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ForbidBlocking {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn dies_make_rats(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token count X = "amount of damage dealt to it this turn" has no
    // script accessor (SelfDies carries no damage-taken amount), so the
    // dynamic count cannot be computed with the demonstrated API.
    Vec::new()
}
