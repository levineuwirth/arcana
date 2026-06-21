//! Pentavus — `{7}` 0/0 Artifact Creature — Construct.
//! "This creature enters with five +1/+1 counters on it."
//! "{1}, Remove a +1/+1 counter from this creature: Create a 1/1
//!   colorless Pentavite artifact creature token with flying."
//! "{1}, Sacrifice a Pentavite: Put a +1/+1 counter on this creature."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pentavus");
    let construct = reg.interner_mut().intern("Construct");
    // Intern "Pentavite" now so the sacrifice-cost filter and the token's
    // subtype resolve to a real symbol.
    let _pentavite = reg.interner_mut().intern("Pentavite");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    // Cost field for "Sacrifice a Pentavite" — chosen permanent of the
    // Pentavite subtype that you control.
    let pentavite_filter = arcana_core::script::subtype_filter(reg, "Pentavite");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_five_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless Pentavite artifact creature token with flying.".into(),
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
                effect: make_pentavite,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice a Pentavite: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice_other: Some(pentavite_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter_to_self,
            }),
    )
}

fn etb_five_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 5,
    }]
}

fn make_pentavite(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pentavite = reg.interner().lookup("Pentavite").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pentavite);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: pentavite,
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

fn add_counter_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
