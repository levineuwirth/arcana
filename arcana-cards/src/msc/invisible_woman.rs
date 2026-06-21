//! Invisible Woman — `{2}{W}` 3/3 legendary Human Hero.
//! "At the beginning of combat on your turn, if you've cast a
//! noncreature spell this turn, create a 0/3 colorless Wall creature
//! token with defender and reach.
//! Whenever you attack, you may pay {R}{G}{W}{U}. When you do, target
//! creature gets +1/+0 until end of turn for each creature you control
//! and can't be blocked this turn."
//!
//! Abilities:
//! 1. PhaseBegins(Combat, You) + intervening-if (cast a noncreature
//!    spell this turn) → create a 0/3 colorless Wall token (defender,
//!    reach).
//! 2. CreatureAttacks (you control) → you may pay {R}{G}{W}{U}; if you
//!    do, target creature gets +1/+0 per creature you control and can't
//!    be blocked this turn. PARTIAL: "whenever you attack" is modelled
//!    per-attacker via CreatureAttacks (fires once per attacking
//!    creature rather than once for the whole attack).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invisible Woman");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_cast_noncreature_spell),
                effect: make_wall_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: may_pay_then_buff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn if_cast_noncreature_spell(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::spells_cast_this_turn(
        s,
        &ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
        you,
    ) >= 1
}

fn make_wall_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wall = reg.interner().lookup("Wall").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: wall,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Defender, KeywordAbility::Reach],
            abilities: vec![],
        },
    }]
}

fn may_pay_then_buff(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "+1/+0 until end of turn for each creature you control"
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}{G}{W}{U}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![
            Effect::Pump {
                target: *id,
                power: n,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            },
            Effect::CantBeBlocked {
                target: *id,
                duration: Duration::EndOfTurn,
            },
        ])),
        else_effect: None,
    }]
}
