//! Kjeldoran Javelineer — `{W}` 1/2 Human Soldier.
//! Cumulative upkeep {1} (GAP — keyword not expressible).
//! {T}: This creature deals damage equal to the number of age counters on
//! it to target attacking or blocking creature.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kjeldoran Javelineer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    // Pre-intern the named "age" counter so the resolver can recover it.
    let _age = reg.interner_mut().intern("age");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        // Cumulative upkeep is not a usable keyword.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "Cumulative upkeep {1}" — upkeep age-counter / pay-or-sacrifice
    // keyword not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature deals damage equal to the number of age \
                       counters on it to target attacking or blocking creature."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().attacking_or_blocking_only(),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: javelin,
            }),
    )
}

fn javelin(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let age = match reg.interner().lookup("age").map(CounterKind::Named) {
        Some(k) => k,
        None => return Vec::new(),
    };
    let n = state.objects.get(ctx.source).map_or(0, |o| o.count_counters(age));
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: n,
    }]
}
