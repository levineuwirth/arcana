//! Foggy Swamp Vinebender — `{3}{G}` 4/3 green Human Plant Ally.
//! "This creature can't be blocked by creatures with power 2 or less."
//! "Waterbend {5}: Put a +1/+1 counter on this creature. Activate only during
//! your turn."
//!
//! The evasion line is a power-filtered static, GAP'd. The Waterbend ability
//! is modeled as a plain {5} mana-cost activated ability that adds a +1/+1
//! counter; Waterbend's convoke-like artifact/creature tap helpers and the
//! "only during your turn" timing window are not expressible — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Foggy Swamp Vinebender");
    let human = reg.interner_mut().intern("Human");
    let plant = reg.interner_mut().intern("Plant");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(plant);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "can't be blocked by creatures with power 2 or less" —
    //      power-filtered evasion not expressible as a static here.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Waterbend {5}: Put a +1/+1 counter on this creature.".into(),
            // GAP: Waterbend artifact/creature tap helpers and the "only
            //      during your turn" timing window are not expressible.
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_counter,
        }),
    )
}

fn add_counter(
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
