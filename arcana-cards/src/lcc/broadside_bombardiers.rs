//! Broadside Bombardiers — `{2}{R}` 2/2 Goblin Pirate with Menace and
//! Haste.
//!
//! Oracle:
//! * Menace, haste.
//! * Boast — Sacrifice another creature or artifact: This creature
//!   deals damage equal to 2 plus the sacrificed permanent's mana
//!   value to any target. (Activate only if this creature attacked
//!   this turn and only once each turn.)

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Broadside Bombardiers");
    let goblin = reg.interner_mut().intern("Goblin");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Boast — Sacrifice another creature or artifact: \
                       This creature deals damage equal to 2 plus the \
                       sacrificed permanent's mana value to any target."
                    .into(),
                cost: ActivationCost {
                    sacrifice_other: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(
                                TypeLine::CREATURE | TypeLine::ARTIFACT,
                            )),
                    ),
                    once_per_turn: true,
                    activation_condition: Some(boast_attacked),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: boast_damage,
            }),
    )
}

/// Boast restriction — "activate only if this creature attacked this
/// turn".
fn boast_attacked(
    s: &GameState,
    src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::source_attacked_this_turn(s, src)
}

fn boast_damage(
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
    // Deals the printed floor of 2 damage.
    // GAP: "plus the sacrificed permanent's mana value" — the cost-
    // sacrificed permanent's mana value is not exposed on
    // ActivationContext, so the dynamic bonus cannot be added.
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 2,
    }]
}
