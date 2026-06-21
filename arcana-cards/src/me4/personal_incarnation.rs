//! Personal Incarnation — `{3}{W}{W}{W}` 6/6 Creature — Avatar
//! Incarnation.
//!
//! Oracle:
//! * "{0}: The next 1 damage that would be dealt to this creature this
//!   turn is dealt to its owner instead. Only this creature's owner may
//!   activate this ability." — redirect the next 1 damage from this
//!   creature to its controller (the engine's RedirectDamage caps at the
//!   first source via ReplacementDuration::EndOfTurn). GAP: the
//!   "only this creature's owner may activate" controller restriction
//!   is not an expressible activation gate; the "next 1 damage" cap is
//!   approximated as a redirect of the next source's damage.
//! * "When this creature dies, its owner loses half their life, rounded
//!   up." — a death trigger. GAP: "half their life rounded up" is a
//!   resolution-time amount derived from the controller's current life,
//!   computable as (life + 1) / 2.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Personal Incarnation");
    let avatar = reg.interner_mut().intern("Avatar");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Only this creature's owner may activate this ability"
                // — no controller-restriction field on activated abilities.
                text: "{0}: The next 1 damage that would be dealt to this creature this turn is dealt to its owner instead.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: redirect_to_owner,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: owner_loses_half,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn redirect_to_owner(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::RedirectDamage {
        from: DamageTarget::Object(ctx.source),
        to: DamageTarget::Player(ctx.controller),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn owner_loses_half(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let life = script::life(state, trig.controller).max(0) as u32;
    let half = (life + 1) / 2;
    vec![Effect::LoseLife {
        player: trig.controller,
        amount: half,
    }]
}
