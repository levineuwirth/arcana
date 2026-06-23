//! Clackbridge Troll — `{3}{B}{B}` 8/8 Troll with Trample and Haste.
//! "When this creature enters, target opponent creates three 0/1 white Goat
//! creature tokens."
//! "At the beginning of combat on your turn, any opponent may sacrifice a
//! creature of their choice. If a player does, tap this creature, you gain
//! 3 life, and you draw a card."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clackbridge Troll");
    let troll = reg.interner_mut().intern("Troll");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    // Pre-intern the token's subtype for the ETB resolver.
    let _goat = reg.interner_mut().intern("Goat");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_goats,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_opponent_may_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_goats(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "target opponent creates three 0/1 white Goat creature tokens."
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let arcana_core::targets::TargetChoice::Player(p) = target else { return Vec::new(); };
    let goat = reg.interner().lookup("Goat").unwrap_or_default();
    let mut goat_subtypes = SubtypeSet::default();
    goat_subtypes.0.insert(goat);
    let token = TokenDefinition {
        name: goat,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: goat_subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: *p, token: token.clone() },
        Effect::CreateToken { controller: *p, token: token.clone() },
        Effect::CreateToken { controller: *p, token },
    ]
}

fn combat_opponent_may_sacrifice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "any opponent may sacrifice a creature of their choice. If a
    //      player does, tap this creature, you gain 3 life, and you draw a
    //      card." — an opponent-optional sacrifice gating a reflexive
    //      payoff is not expressible: OptionalPayment pays mana/life by the
    //      controller (not an opponent sacrificing), and no primitive
    //      conditions a payoff on whether an opponent chose to sacrifice.
    Vec::new()
}
