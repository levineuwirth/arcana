//! Tender Wildguide — `{1}{G}` 2/2 Possum Druid.
//! Offspring {2}. "{T}: Add one mana of any color." "{T}: Put a +1/+1
//! counter on this creature."

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
    let name = reg.interner_mut().intern("Tender Wildguide");
    let possum = reg.interner_mut().intern("Possum");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(possum);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Offspring {2} — not in the supported keyword surface (no cast-
        //      time additional-cost rider / token-copy ETB hook).
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "{T}: Add one mana of any color." — ManaUnit::plain needs a fixed
    //      color; there is no "add one mana of any color" choice primitive in
    //      the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Put a +1/+1 counter on this creature.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: counter_self,
        }),
    )
}

fn counter_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
