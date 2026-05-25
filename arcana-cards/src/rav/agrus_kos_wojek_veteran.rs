//! Agrus Kos, Wojek Veteran — `{3}{R}{W}` 3/3 legendary red-white Human
//! Soldier.
//! "Whenever Agrus Kos attacks, attacking red creatures get +2/+0 and
//! attacking white creatures get +0/+2 until end of turn."
//! GAP: "attacking red/white creatures" — no script helper for attacking
//! creatures; using all red/white creatures you control as approximation.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Agrus Kos, Wojek Veteran");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_pump_red_white,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_pump_red_white(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "attacking" qualifier omitted — pumping all red/white you control.
    let red_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_colors(ColorSet::red());
    let white_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_colors(ColorSet::white());
    let red_ids = script::ids_matching(state, &red_filter, trig.controller);
    let white_ids = script::ids_matching(state, &white_filter, trig.controller);
    vec![
        Effect::ForEach {
            targets: red_ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 2,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
        Effect::ForEach {
            targets: white_ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 0,
                toughness: 2,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
    ]
}
