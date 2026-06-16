//! Golden-Tail Trainer — `{1}{G}{W}` 1/3 Fox Samurai.
//! Aura/Equipment spells you cast cost {X} less (static cost reduction —
//! gapped). Whenever it attacks, other modified creatures you control get
//! +X/+X until end of turn, X = its power. (The "modified" restriction and
//! the "other" self-exclusion are not expressible; modeled as a +X/+X pump
//! to each creature you control.)

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Golden-Tail Trainer");
    let fox = reg.interner_mut().intern("Fox");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Aura and Equipment spells you cast cost {X} less to cast" —
        // static spell cost reduction not expressible here.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: filter cannot restrict to "modified" creatures nor exclude
            // this creature; pumps every creature you control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_modified,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_modified(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::power_of(state, trig.source).max(0) as i32;
    if x == 0 {
        return Vec::new();
    }
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
