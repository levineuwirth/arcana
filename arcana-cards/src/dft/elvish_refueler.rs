//! Elvish Refueler — `{2}{G}` 2/3 Elf Druid.
//! Exhaust mechanic: during your turn, as long as you haven't activated an
//! exhaust ability this turn, you may activate exhaust abilities as though they
//! haven't been activated.
//! Exhaust — `{1}{G}`: Put a +1/+1 counter on this creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elvish Refueler");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Exhaust" keyword (activate-only-once) is not a KeywordAbility
        // variant; the static reset rider is unmodeled too.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP static: "during your turn, as long as you haven't activated an
            // exhaust ability this turn, you may activate exhaust abilities as
            // though they haven't been activated" — exhaust-reset is unexpressible.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {1}{G}: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exhaust_counter,
            }),
    )
}

fn exhaust_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exhaust "activate only once" gating is not a cost/precondition field.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
