//! Butch DeLoria, Tunnel Snake — `{1}{B}` 2/2 Legendary Human Rogue.
//! Menace.
//! "Tunnel Snakes Rule!" — Whenever Butch attacks, it gets +1/+1 until end of
//! turn for each other Rogue and/or Snake you control.
//! `{1}{B}`: Put a menace counter on another target creature. It becomes a Rogue
//! in addition to its other types.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Butch DeLoria, Tunnel Snake");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Put a menace counter on another target creature. It becomes a Rogue in addition to its other types.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_menace_rogue,
            }),
    )
}

fn on_attacks(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // +1/+1 for each other Rogue and/or Snake you control.
    let rogues = script::count_matching(
        state,
        &script::subtype_filter(reg, "Rogue").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let snakes = script::count_matching(
        state,
        &script::subtype_filter(reg, "Snake").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    // Total others; subtract 1 for Butch himself (a Rogue counted above).
    let n = (rogues + snakes).saturating_sub(1) as i32;
    if n <= 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: n,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn grant_menace_rogue(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let menace = reg.interner().lookup("menace").map(CounterKind::Named);
    let mut out = Vec::new();
    if let Some(kind) = menace {
        out.push(Effect::AddCounters {
            target: *id,
            kind,
            count: 1,
        });
    }
    // It becomes a Rogue in addition to its other types — subtype add is not an
    // expressible Effect variant (AddType is for card types only).
    // GAP: "becomes a Rogue in addition to its other types" — no subtype-grant effect.
    out
}
