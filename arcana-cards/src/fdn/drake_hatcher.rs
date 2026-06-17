//! Drake Hatcher — `{1}{U}` 1/3 Human Wizard with Vigilance (and Prowess).
//! "Whenever this creature deals combat damage to a player, put that many
//! incubation counters on it. Remove three incubation counters from this
//! creature: Create a 2/2 blue Drake creature token with flying."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drake Hatcher");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let incubation = reg.interner_mut().intern("incubation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Prowess is not in the usable KeywordAbility surface for this class.
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: add_incubation_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove three incubation counters from this creature: Create a 2/2 blue Drake creature token with flying.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(incubation), 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_drake_token,
            }),
    )
}

fn add_incubation_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let Some(incubation) = reg.interner().lookup("incubation").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: incubation,
        count: n,
    }]
}

fn make_drake_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let drake = reg.interner().lookup("Drake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: drake,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
