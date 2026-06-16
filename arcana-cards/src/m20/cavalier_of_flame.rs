//! Cavalier of Flame — `{2}{R}{R}{R}` 6/5 Elemental Knight.
//!
//! * `{1}{R}: Creatures you control get +1/+0 and gain haste until end of turn.`
//!   (activated; dynamic pump-all over your creatures via `Effect::ForEach` —
//!   each id gets the boxed `Pump` retargeted, preserving the granted Haste).
//! * "When this creature enters, discard any number of cards, then draw that
//!   many cards." — GAP: no coupled discard-X-then-draw-X primitive (`Discard`
//!   has a fixed count and no draw-the-count rider).
//! * "When this creature dies, it deals X damage to each opponent and each
//!   planeswalker they control, where X = land cards in your graveyard." — the
//!   damage to each opponent is expressible (X via `script::graveyard_matching`
//!   over a land filter); the per-planeswalker damage is GAP'd (no board id
//!   enumeration of opponents' planeswalkers as a damage-target set here).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cavalier of Flame");
    let elemental = reg.interner_mut().intern("Elemental");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Creatures you control get +1/+0 and gain haste until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_your_creatures,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_burn_opponents,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// `{1}{R}`: each creature you control gets +1/+0 and gains haste EOT.
fn pump_your_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        }),
    }]
}

/// ETB: "discard any number of cards, then draw that many cards."
fn etb_loot(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no coupled discard-any-number-then-draw-that-many primitive
    // (Effect::Discard has a fixed count and there is no draw-equal-to-discarded rider).
    Vec::new()
}

/// Dies: deals X to each opponent, X = land cards in your graveyard.
fn dies_burn_opponents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let land_filter = ObjectFilter {
        types: Some(TypeLine::LAND.into()),
        ..ObjectFilter::default()
    };
    let x = script::graveyard_matching(state, &land_filter, trig.controller, trig.controller);
    if x == 0 {
        return Vec::new();
    }
    // GAP: per-planeswalker damage ("and each planeswalker they control") is not
    // emitted — only the each-opponent half is expressed here.
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: x,
        })
        .collect()
}
