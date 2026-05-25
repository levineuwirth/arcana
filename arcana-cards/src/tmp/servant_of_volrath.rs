//! Servant of Volrath — `{2}{B}` 3/3 black Minion.
//! "When this creature leaves the battlefield, sacrifice a creature."
//! GAP: "leaves the battlefield" trigger not in the engine trigger condition catalog;
//! using SelfDies as closest (fires on death only, not on other zone changes like bounce).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Servant of Volrath");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "leaves the battlefield" trigger not in catalog; using SelfDies
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_leaves_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_leaves_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
