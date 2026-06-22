//! Liara of the Flaming Fist — `{R}{W}` 2/2 Legendary Creature — Human
//! Soldier.
//! "At the beginning of each combat, each creature you control gets +1/+1
//! until end of turn if it has the same name as another creature you
//! control or a creature card in your graveyard."
//! "{1}{R}{W}: Another target nontoken creature you control gains first
//! strike and double team until end of turn. Activate only as a sorcery
//! and only once."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liara of the Flaming Fist");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: combat_name_match_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{W}: Another target nontoken creature you control \
                       gains first strike and double team until end of turn. \
                       Activate only as a sorcery and only once."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{W}").expect("valid cost"),
                    // "only once" (per game) approximated by once-per-turn —
                    // the closest available gate.
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You)
                            .nontoken(),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_first_strike,
            }),
    )
}

fn combat_name_match_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: per-creature conditional pump gated on "same name as another
    //      creature you control or a creature card in your graveyard" —
    //      no script:: name-matching predicate to compute the affected set.
    Vec::new()
}

fn grant_first_strike(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // "double team" is not in the usable keyword surface — only the first
    // strike grant is expressible.
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::FirstStrike,
        duration: Duration::EndOfTurn,
    }]
}
