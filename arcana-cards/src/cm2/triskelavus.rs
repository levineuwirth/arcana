//! Triskelavus — `{7}` 1/1 Artifact Creature — Construct.
//! Flying.
//! This creature enters with three +1/+1 counters on it.
//! {1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless
//! Triskelavite artifact creature token with flying. It has "Sacrifice this
//! token: This token deals 1 damage to any target."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Triskelavus");
    let construct = reg.interner_mut().intern("Construct");
    // Pre-intern the token subtype so the resolver can look it up.
    let _ = reg.interner_mut().intern("Triskelavite");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_three_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless \
                       Triskelavite artifact creature token with flying. It has \"Sacrifice this \
                       token: This token deals 1 damage to any target.\""
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_triskelavite,
            }),
    )
}

fn etb_three_counters(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}

fn make_triskelavite(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let name = reg.interner().lookup("Triskelavite").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(name);
    // GAP: the token's granted "Sacrifice this token: deals 1 damage to any
    // target" activated ability is not expressible on a TokenDefinition
    // (abilities vec carries triggered, not activated, abilities); the bare
    // 1/1 flying artifact token is created.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
