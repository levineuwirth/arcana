//! Thran Vigil — `{1}{B}` enchantment.
//! "Whenever one or more artifact and/or creature cards leave your
//! graveyard during your turn, put a +1/+1 counter on target creature
//! you control."
//!
//! Wired as a graveyard-to-battlefield `ZoneChange` (the most common
//! leave path). GAPs: "leave your graveyard" to ANY zone is not
//! expressible (`ZoneChange.to` is a single required zone), the
//! "your graveyard" / "during your turn" constraints have no hooks,
//! and "one or more … cards" batches into a single trigger whereas
//! this fires per card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thran Vigil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "leave your graveyard [to any zone]
                // during your turn": ZoneChange requires one destination
                // zone (battlefield chosen as the common reanimation
                // path), has no owner ("your graveyard") or
                // during-your-turn constraint, and fires per card
                // rather than per batch.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::ARTIFACT | TypeLine::CREATURE,
                    )),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: grow_a_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…put a +1/+1 counter on target creature you control."
fn grow_a_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
