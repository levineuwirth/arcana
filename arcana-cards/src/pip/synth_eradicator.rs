//! Synth Eradicator — `{2}{R}` 3/3 red Artifact Creature — Synth
//! Soldier with Haste.
//!
//! * Haste — keyword line.
//! * "Whenever this creature attacks, exile the top card of your
//!   library. You may get {E}{E}. If you don't, you may play that card
//!   this turn." — modeled with `Effect::ImpulseExile` (exile the top
//!   card, gain permission to play it this turn). The energy-vs-play
//!   branch ("you may get {E}{E} INSTEAD of the play permission") is
//!   GAP'd: there is no conditional energy-gain-or-play primitive.
//! * "{T}, Pay {E}{E}{E}: This creature deals 3 damage to any target."
//!   — the tap cost + any-target 3-damage effect are emitted faithfully;
//!   the `Pay {E}{E}{E}` energy cost is GAP'd (ActivationCost has no
//!   energy field).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Synth Eradicator");
    let synth = reg.interner_mut().intern("Synth");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(synth);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Pay {E}{E}{E}: This creature deals 3 damage to any target.".into(),
                cost: ActivationCost {
                    tap: true,
                    // GAP: "Pay {E}{E}{E}" — ActivationCost has no energy
                    // cost field; only the {T} portion is modeled.
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_three,
            }),
    )
}

fn attacks_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "exile the top card of your library ... you may play that card
    // this turn." GAP: the "you may get {E}{E} instead" energy branch
    // is not expressible (no conditional energy-or-play primitive).
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}

fn ping_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 3,
    }]
}
