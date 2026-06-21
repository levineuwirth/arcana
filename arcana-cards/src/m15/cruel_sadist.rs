//! Cruel Sadist — `{B}` 1/1 Human Assassin.
//! `{B}, {T}, Pay 1 life: Put a +1/+1 counter on this creature.`
//! `{2}{B}, {T}, Remove X +1/+1 counters from this creature: It deals
//! X damage to target creature.`
//!
//! Two activated abilities. The first grows the creature; the second
//! removes counters as a cost and deals damage equal to the number
//! removed. The "X" removal/spend on the second ability is a variable
//! counter removal not expressible via the fixed `remove_self_counter`
//! shape, so that ability is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Sadist");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}, Pay 1 life: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    tap: true,
                    life: 1,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grow_self,
            })
            // GAP: "{2}{B}, {T}, Remove X +1/+1 counters: It deals X damage to
            // target creature." The variable counter-removal cost (Remove X) is
            // not expressible via the fixed `remove_self_counter: (kind, n)` cost
            // field, and the damage amount keys off that chosen X.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, {T}, Remove X +1/+1 counters from this creature: It deals X damage to target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_remove_x_damage,
            }),
    )
}

fn grow_self(
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

fn gap_remove_x_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: variable "Remove X +1/+1 counters" cost + X-scaled damage not
    // expressible with the fixed ActivationCost counter-removal shape.
    Vec::new()
}
