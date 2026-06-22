//! Threefold Thunderhulk — `{7}` 0/0 Artifact Creature — Gnome.
//!
//! Oracle:
//! * This creature enters with three +1/+1 counters on it. (enters-with
//!   replacement — GAP'd; no enters-with-counters mechanism in this shape.)
//! * Whenever this creature enters or attacks, create a number of 1/1 colorless
//!   Gnome artifact creature tokens equal to its power.
//! * {2}, Sacrifice another artifact: Put a +1/+1 counter on this creature.
//!
//! The "enters or attacks" clause is decomposed into two triggered abilities
//! (SelfEntersBattlefield and SelfAttacks), each creating Gnome tokens equal to
//! this creature's current power (`script::power_of`). The sacrifice activation
//! uses `sacrifice_other` over artifacts you control and adds a +1/+1 counter.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Threefold Thunderhulk");
    let gnome = reg.interner_mut().intern("Gnome");
    // Token subtype (Gnome) pre-interned.
    let _gnome_token = reg.interner_mut().intern("Gnome");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);

    // GAP: "This creature enters with three +1/+1 counters on it." — an
    // enters-with-counters replacement with no mechanism in this shape.
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
                effect: make_gnomes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_gnomes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Sacrifice another artifact: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter,
            }),
    )
}

fn make_gnomes(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Create 1/1 colorless Gnome artifact-creature tokens equal to this
    // creature's power.
    let n = script::power_of(state, trig.source).max(0) as u32;
    let gnome = reg.interner().lookup("Gnome").unwrap_or_default();
    let mut token_subs = SubtypeSet::default();
    token_subs.0.insert(gnome);
    let token = TokenDefinition {
        name: gnome,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: token_subs,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}

fn add_counter(
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
