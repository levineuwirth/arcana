//! Bojuka Brigand — `{1}{B}` 1/1 Human Warrior Ally.
//! "This creature can't block." — GAP: static can't-block continuous
//!  ability with no trigger/cost is not expressible here.
//! "Whenever this creature or another Ally you control enters, you may put
//!  a +1/+1 counter on this creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bojuka Brigand");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(ally);

    // "this creature or another Ally you control enters" — an Ally you
    // control entering. The source itself is an Ally you control, so this
    // filter covers both the self and the "another Ally" cases.
    let ally_filter = script::subtype_filter(reg, "Ally")
        .controlled_by(ControllerConstraint::You);

    // GAP: "This creature can't block." — pure static, no trigger/cost.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ally_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: put_counter_on_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn put_counter_on_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may put a +1/+1 counter" — best-effort: always put the counter.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
