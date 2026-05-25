//! Terra Ravager — `{2}{R}{R}` 0/4 red Elemental Beast. "Whenever this creature attacks,
//! it gets +X/+0 until end of turn, where X is the number of lands defending player controls."
//! SelfAttacks trigger; X = lands of defending player via defending_player() + count_matching.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Terra Ravager");
    let elemental = reg.interner_mut().intern("Elemental");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(defending) = trig.defending_player() else { return Vec::new(); };
    let x = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into()),
        defending,
    ) as i32;
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: x,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
