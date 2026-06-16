//! Human Torch — `{3}{R}` 3/2 Legendary Human Hero.
//! At the beginning of combat on your turn, if you've cast a noncreature
//! spell this turn, Human Torch gains flying, double strike, and haste
//! until end of turn.
//! Whenever Human Torch attacks, you may pay {R}{G}{W}{U}. If you do,
//! until end of turn, whenever he deals combat damage to an opponent, he
//! deals that much damage to each other opponent.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Human Torch");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if you've cast a noncreature spell this turn"
            // — no conditions:: predicate for "cast a noncreature spell this
            // turn"; firing unconditionally is the closest, gated only by phase.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_combat_keywords,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "you may pay {R}{G}{W}{U}. If you do, until end of turn,
            // whenever he deals combat damage to an opponent, he deals that
            // much to each other opponent." OptionalPayment exists, but the
            // payoff is a delayed combat-damage redistribution across opponents
            // that has no expressible Effect; the whole reflexive ability is a gap.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pay_payoff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_combat_keywords(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::DoubleStrike,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn attack_pay_payoff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: pay {R}{G}{W}{U} → grant a delayed "deals combat damage to an
    // opponent, deal that much to each OTHER opponent" rider; the payoff has
    // no expressible Effect (per-opponent damage echo from combat damage).
    Vec::new()
}
