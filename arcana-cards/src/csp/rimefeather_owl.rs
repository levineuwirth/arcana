//! Rimefeather Owl — `{5}{U}{U}` Snow Creature — Bird with Flying.
//! P/T are each equal to the number of snow permanents on the battlefield.
//! "{1}{S}: Put an ice counter on target permanent."
//! "Permanents with ice counters on them are snow."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rimefeather Owl");
    let bird = reg.interner_mut().intern("Bird");
    let _ice = reg.interner_mut().intern("ice");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    // GAP: characteristic-defining */* (power/toughness = number of snow
    // permanents on the battlefield) is a static CDA not expressible as a
    // Fixed PtValue; using 0/0 base.
    // GAP: static "Permanents with ice counters on them are snow" — no
    // continuous snow-granting effect in the demonstrated API.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{S}: Put an ice counter on target permanent.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_ice_counter,
            }),
    )
}

fn put_ice_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let kind = reg
        .interner()
        .lookup("ice")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    vec![Effect::AddCounters { target: *id, kind, count: 1 }]
}
