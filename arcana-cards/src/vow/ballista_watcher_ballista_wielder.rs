//! Ballista Watcher // Ballista Wielder — `{2}{R}{R}` red Creature — Human
//! Soldier Werewolf 4/3. Transforms to Werewolf (back face).
//!
//! Front face: {2}{R}, {T}: This creature deals 1 damage to any target.
//! Daybound (If a player casts no spells during their own turn, it becomes
//! night next turn.)
//! GAP: Daybound/Nightbound day/night cycle not modeled; transform wired via
//!   upkeep trigger unconditionally.
//!
//! Back face — Ballista Wielder (Werewolf):
//! {2}{R}: This creature deals 1 damage to any target. A creature dealt damage
//! this way can't block this turn.
//! Nightbound (If a player casts at least two spells during their own turn, it
//! becomes day next turn.)
//! GAP: back-face activated ability ({2}{R}: deal 1 damage + can't block)
//!   not auto-installed on transform.
//! GAP: "can't block this turn" on a creature dealt damage not expressible
//!   as a follow-on effect (no Effect::CantBlock targeting a specific creature
//!   dealt damage by this effect; CantBeBlocked is for evasion, not prevention).
//! GAP: back-face-only triggered abilities not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ballista Watcher");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(soldier_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ballista Wielder");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {2}{R}, {T}: deal 1 damage to any target.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, {T}: This creature deals 1 damage to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: front_shoot,
            })
            // GAP: Daybound transform condition not modeled; wired as unconditional
            // upkeep trigger as closest approximation.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: daybound_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face activated ability ({2}{R}: deal 1 + can't block)
            //   not auto-installed on transform.
            // GAP: back-face Nightbound transform not modeled.
    )
}

fn front_shoot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        target: dt,
        amount: 1,
        source: ctx.source,
    }]
}

fn daybound_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only fire when Daybound condition is met (no spells cast
    // during a player's own turn). Fires unconditionally here.
    vec![Effect::Transform { target: trig.source }]
}
