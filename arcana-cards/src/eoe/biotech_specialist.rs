//! Biotech Specialist — `{R}{G}` 1/3 Insect Scientist.
//!
//! Oracle:
//! * When this creature enters, create a Lander token. (Artifact with
//!   "{2}, {T}, Sacrifice this token: Search your library for a basic land
//!   card, put it onto the battlefield tapped, then shuffle.")
//! * Whenever you sacrifice an artifact, this creature deals 2 damage to
//!   target opponent.
//!
//! The ETB mints a bare Lander artifact token; its printed sacrifice-tutor
//! activated ability is GAP'd (no way to attach a token activation here). The
//! sacrifice trigger is fully wired against an artifact-you-control filter and
//! deals 2 damage to a target opponent (a player).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Biotech Specialist");
    let insect = reg.interner_mut().intern("Insect");
    let scientist = reg.interner_mut().intern("Scientist");
    let _lander = reg.interner_mut().intern("Lander");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_lander,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: damage_target_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn etb_make_lander(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let lander = reg.interner().lookup("Lander").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lander);
    // GAP: the Lander's printed "{2}, {T}, Sacrifice: search for a basic land,
    // put it tapped, shuffle" activated ability cannot be attached to a token
    // here — only the bare artifact token is minted.
    let token = TokenDefinition {
        name: lander,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn damage_target_opponent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(*p),
        amount: 2,
    }]
}
