//! Warden of the First Tree — `{G}` 1/1 Human (G).
//!
//! Three escalating "level-up" activated abilities:
//! * "{1}{W/B}: This creature becomes a Human Warrior with base power and
//!   toughness 3/3." → SetBasePT(3/3, permanent). GAP: adding the Warrior subtype
//!   has no demonstrated effect primitive (no AddSubtype), so the type change is
//!   not modeled — only the base 3/3 is set.
//! * "{2}{W/B}{W/B}: If this creature is a Warrior, it becomes a Human Spirit
//!   Warrior with trample and lifelink." → GrantKeyword Trample + Lifelink
//!   (permanent). GAP: the "if this is a Warrior" gate and the Spirit-subtype
//!   addition are not expressible; the keyword grants are wired unconditionally.
//! * "{3}{W/B}{W/B}{W/B}: If this creature is a Spirit, put five +1/+1 counters
//!   on it." → AddCounters(+1/+1, 5). GAP: the "if this is a Spirit" gate is not
//!   expressible; the counters are added unconditionally.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warden of the First Tree");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W/B}: This creature becomes a Human Warrior with base power and toughness 3/3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W/B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_warrior,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W/B}{W/B}: If this creature is a Warrior, it becomes a Human Spirit Warrior with trample and lifelink.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W/B}{W/B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_spirit_warrior,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W/B}{W/B}{W/B}: If this creature is a Spirit, put five +1/+1 counters on it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W/B}{W/B}{W/B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_five_counters,
            }),
    )
}

fn become_warrior(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: adding the Warrior subtype is not expressible; only base P/T is set.
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 3,
        toughness: 3,
        duration: Duration::Permanent,
    }]
}

fn become_spirit_warrior(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if this is a Warrior" gate and Spirit-subtype addition not expressible.
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::Permanent,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::Permanent,
        },
    ]
}

fn add_five_counters(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if this is a Spirit" gate not expressible; counters added unconditionally.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 5,
    }]
}
