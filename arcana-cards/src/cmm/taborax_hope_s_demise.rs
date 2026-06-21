//! Taborax, Hope's Demise — `{2}{B}` 2/2 Legendary Demon Cleric with
//! Flying.
//!
//! * Flying (keyword).
//! * Taborax has lifelink as long as it has five or more +1/+1 counters
//!   on it. (static conditional keyword — GAP'd below)
//! * Whenever another nontoken creature you control dies, put a +1/+1
//!   counter on Taborax. If that creature was a Cleric, you may draw a
//!   card. If you do, you lose 1 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Taborax, Hope's Demise");
    let demon = reg.interner_mut().intern("Demon");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP (static): "Taborax has lifelink as long as it has five or more
    // +1/+1 counters on it." — a conditional self-static keyword grant; not
    // expressible as a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_creature_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If that creature was a Cleric, you may draw a card. If you do,
    // you lose 1 life." — no documented helper to test the dying object's
    // subtype; only the unconditional +1/+1 counter is emitted.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
