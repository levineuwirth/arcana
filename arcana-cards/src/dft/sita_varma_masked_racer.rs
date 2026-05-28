//! Sita Varma, Masked Racer — `{G}{U}` 2/3 legendary green/blue Human Rogue.
//! Exhaust — "{X}{G}{G}{U}: Put X +1/+1 counters on Sita Varma. Then you may
//! have the base power and toughness of each other creature you control become
//! equal to Sita Varma's power until end of turn."
//! GAP: Exhaust mechanic (activate only once) is not modeled; "base power
//! and toughness = Sita's power" for all creatures is not in the Effect
//! catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sita Varma, Masked Racer");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {X}{G}{G}{U}: Put X +1/+1 counters on Sita Varma. Then you may set base P/T of other creatures to Sita's power until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{G}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exhaust_effect,
            }),
    )
}

fn exhaust_effect(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    // GAP: "base P/T of other creatures = Sita's power" not in catalog.
    if x == 0 { return Vec::new(); }
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: x,
    }]
}
