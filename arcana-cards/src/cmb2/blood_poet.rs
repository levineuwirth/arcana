//! Blood Poet — `{2}{B}` 3/2 black Vampire Cleric.
//!
//! Rules text:
//! * Spark (Activate spark abilities by spending or giving yourself spark
//!   counters. Activate only one spark ability per turn, and only as a
//!   sorcery.)
//! * +1: Blood Poet gains lifelink until end of turn.
//! * −3: Target opponent discards a card. You gain life equal to its
//!   converted mana cost.
//!
//! "Convert" is not in the usable keyword surface → keywords: vec![].
//! Spark is a bespoke loyalty-like mechanic with no dedicated engine
//! pathway for creatures, but its two abilities are modeled best-effort as
//! activated abilities keyed on a named "spark" counter on this creature:
//!  • "+1" adds a spark counter (once per turn) and grants this creature
//!    lifelink until end of turn.
//!  • "−3" removes three spark counters (once per turn) and makes a target
//!    opponent discard a card.
//! FIDELITY GAPs: the shared "only one spark ability per turn" limit is
//! approximated per-ability via once_per_turn; the sorcery-speed timing is
//! the default; and "you gain life equal to its converted mana value" is
//! GAP'd (the discarded card's mana value isn't observable in the resolver).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blood Poet");
    let vampire = reg.interner_mut().intern("Vampire");
    let cleric = reg.interner_mut().intern("Cleric");
    let spark = reg.interner_mut().intern("spark");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Blood Poet gains lifelink until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Named(spark), 1)),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: spark_plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Target opponent discards a card. You gain life \
                       equal to its converted mana cost."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(spark), 3)),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: spark_minus_three,
            }),
    )
}

fn spark_plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Lifelink,
        duration: Duration::EndOfTurn,
    }]
}

fn spark_minus_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // PARTIAL: target opponent discards a card. "You gain life equal to its
    // converted mana value" is GAP'd — the discarded card's mana value isn't
    // observable in the resolver.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
