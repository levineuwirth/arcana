//! Rust Harvester — `{R}` 1/1 Artifact Creature — Robot.
//! Menace.
//! "{2}, {T}, Exile an artifact card from your graveyard: Put a +1/+1 counter
//! on this creature, then it deals damage equal to its power to any target."
//!
//! The activation cost models the {2} and {T} portions; the additional
//! "exile an artifact card from your graveyard" cost is not expressible
//! (no exile-other-from-graveyard cost field) and is noted as a GAP. The
//! effect (grow, then deal damage equal to power to any target) is faithful.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rust Harvester");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: the "Exile an artifact card from your graveyard" portion of
            // the activation cost is not expressible (no exile-other-from-
            // graveyard cost field); only {2}, {T} are modeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Exile an artifact card from your graveyard: Put a +1/+1 counter on this creature, then it deals damage equal to its power to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grow_and_blast,
            }),
    )
}

fn grow_and_blast(
    state: &GameState,
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
    };
    // The damage uses the creature's power AFTER the +1/+1 counter is added.
    let power = (script::power_of(state, ctx.source).max(0) as u32) + 1;
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DealDamage {
            source: ctx.source,
            target: dt,
            amount: power,
        },
    ]
}
