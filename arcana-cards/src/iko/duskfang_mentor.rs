//! Duskfang Mentor — `{2}{B}` 1/3 Human Cleric.
//! When this creature enters, put a lifelink counter on target non-Human
//! creature you control.
//! `{1}{B}, {T}: Put a +1/+1 counter on each creature you control with lifelink.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duskfang Mentor");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // "non-Human creature you control" — exclude the Human subtype.
    let non_human = ObjectFilter::creature()
        .without_subtype_sym(human)
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_lifelink_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(non_human),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, {T}: Put a +1/+1 counter on each creature you control with lifelink.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_lifelinkers,
            }),
    )
}

fn etb_lifelink_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let kind = reg
        .interner()
        .lookup("lifelink")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::PlusOnePlusOne);
    vec![Effect::AddCounters {
        target: *id,
        kind,
        count: 1,
    }]
}

fn counter_lifelinkers(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/+1 counter on each creature you control WITH LIFELINK" — the
    // documented ObjectFilter surface has no keyword predicate, so the set of
    // lifelink-bearing creatures cannot be filtered; emitting a counter on every
    // creature you control would be materially wrong, so the effect is omitted.
    Vec::new()
}
