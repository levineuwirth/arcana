//! Sunscorched Desert — Land — Desert (Amonkhet).
//! "When this land enters, it deals 1 damage to target player or
//! planeswalker." and "{T}: Add {C}."
//!
//! Fidelity note: "target player or planeswalker" is declared via
//! `any_target()` (which also admits creatures — a documented widening;
//! no player-or-planeswalker-only TargetFilter exists).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunscorched Desert");
    let desert = reg.interner_mut().intern("Desert");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(desert);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_ping,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target player or planeswalker" — any_target() also
                // admits creatures; no narrower TargetFilter exists.
                target_requirements: vec![TargetRequirement::any_target()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            }),
    )
}

fn etb_ping(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let damage_target = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        target: damage_target,
        amount: 1,
        source: trig.source,
    }]
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}
