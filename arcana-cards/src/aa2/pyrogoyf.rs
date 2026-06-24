//! Pyrogoyf — `{3}{R}` */1+* Lhurgoyf.
//!
//! Oracle:
//! * Pyrogoyf's power is equal to the number of card types among cards
//!   in all graveyards and its toughness is equal to that number plus 1.
//!   — a characteristic-defining ability, wired via a self_pt_cda
//!   installed on ETB: the compute ORs the TypeLine bits of every card
//!   in every graveyard and counts the distinct card-type bits `n`,
//!   setting base P/T to `(n, n + 1)`.
//! * Whenever this creature or another Lhurgoyf creature you control
//!   enters, that creature deals damage equal to its power to any
//!   target. — expressed via a battlefield ZoneChange watching
//!   Lhurgoyf creatures you control (the filter matches this creature
//!   too, covering the "this creature or another" wording).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::{Zone, ZoneKind};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pyrogoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // P/T are a CDA — power = card types among all graveyards,
        // toughness = that + 1 — resolved at Layer 7a by install_cda.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        ..Default::default()
    };

    let lhurgoyf_filter = script::subtype_filter(reg, "Lhurgoyf")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: lhurgoyf_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: entered_deal_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            })
            // CDA: power = card types among all graveyards, toughness = +1.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = number of card types among cards in all graveyards;
// toughness = that number plus 1.
fn cda_pt(s: &GameState, _source: ObjectId) -> (i32, i32) {
    // The eight real card types (Kindred is not a card type for this count).
    const CARD_TYPE_BITS: u16 = TypeLine::CREATURE
        | TypeLine::INSTANT
        | TypeLine::SORCERY
        | TypeLine::ENCHANTMENT
        | TypeLine::ARTIFACT
        | TypeLine::LAND
        | TypeLine::PLANESWALKER
        | TypeLine::BATTLE;
    let mut seen: u16 = 0;
    for o in s.objects.objects_in_zone_kind(ZoneKind::Graveyard) {
        seen |= o.characteristics.types.0 & CARD_TYPE_BITS;
    }
    let n = seen.count_ones() as i32;
    (n, n + 1)
}

fn entered_deal_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let entering = trig.entering_object().unwrap_or(trig.source);
    let amount = script::power_of(state, entering).max(0) as u32;
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: entering,
        target: dt,
        amount,
    }]
}
