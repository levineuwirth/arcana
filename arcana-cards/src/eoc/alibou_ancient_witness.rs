//! Alibou, Ancient Witness — `{3}{R}{W}` 4/5 Legendary Artifact Creature — Golem.
//! "Other artifact creatures you control have haste.
//!  Whenever one or more artifact creatures you control attack, Alibou deals X damage
//!  to any target and you scry X, where X is the number of tapped artifacts you
//!  control."
//!
//! The "other artifact creatures you control have haste" static is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alibou, Ancient Witness");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static "Other artifact creatures you control have haste." — a board-wide
    // keyword-granting static is not expressible from the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars)
            // "one or more artifact creatures you control attack" — the engine fires
            // CreatureAttacks per attacker; the once-per-batch semantics are an
            // approximation (over-fires when multiple artifact creatures attack).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_artifact_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn on_artifact_attack(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You)
            .tapped_only(),
        trig.controller,
    );
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![
        Effect::DealDamage { source: trig.source, target: dt, amount: x },
        Effect::Scry { player: trig.controller, count: x },
    ]
}
