//! Axgard Braggart — `{3}{W}` 3/3 white Dwarf Warrior.
//! "Boast — {1}{W}: Untap this creature. Put a +1/+1 counter on it.
//! (Activate only if this creature attacked this turn and only once each turn.)"
//!
//! The "attacked this turn" half of Boast is enforced via
//! `ActivationCost::activation_condition` + `conditions::source_attacked_this_turn`;
//! "only once each turn" via `once_per_turn`.

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
    let name = reg.interner_mut().intern("Axgard Braggart");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Boast — {1}{W}: Untap this creature. Put a +1/+1 counter on it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").unwrap(),
                    // Boast: only if this creature attacked this turn.
                    activation_condition: Some(|s, src, _you, _reg| {
                        arcana_core::conditions::source_attacked_this_turn(s, src)
                    }),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: boast_ability,
            }),
    )
}

fn boast_ability(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Untap { target: ctx.source },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
