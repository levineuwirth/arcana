//! Duergar Mine-Captain — `{2}{R/W}` 2/1 red/white Dwarf Soldier.
//! "{1}{R/W}, {Q}: Attacking creatures get +1/+0 until end of turn."
//! ({Q} is the untap symbol.)
//!
//! GAP: "{Q}" untap symbol — no ActivationCost field for untap-as-cost.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duergar Mine-Captain");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R/W}, {Q}: Attacking creatures get +1/+0 until end of turn.".into(),
                // GAP: "{Q}" untap-as-cost not in ActivationCost.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_attackers,
            }),
    )
}

fn pump_attackers(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
