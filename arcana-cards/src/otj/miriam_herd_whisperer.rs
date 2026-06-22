//! Miriam, Herd Whisperer — `{G}{W}` 3/2 Legendary Creature — Human Druid.
//! "During your turn, Mounts and Vehicles you control have hexproof.
//!  Whenever a Mount or Vehicle you control attacks, put a +1/+1 counter
//!  on it."
//!
//! The first line is a conditional STATIC granting hexproof during your
//! turn — not a triggered/activated ability and not expressible here, so
//! GAP'd. The second line is a `CreatureAttacks` trigger filtered to
//! Mounts and Vehicles you control; on resolution it puts a +1/+1 counter
//! on the attacking permanent (read via `trig.attacking_creature()`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miriam, Herd Whisperer");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mount = reg.interner_mut().intern("Mount");
    let vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "During your turn, Mounts and Vehicles you control
    // have hexproof." A conditional continuous static, not a
    // triggered/activated ability; not expressible here.

    let attack_filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![mount, vehicle]);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: attack_filter,
            },
            intervening_if: None,
            effect: counter_on_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_on_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
