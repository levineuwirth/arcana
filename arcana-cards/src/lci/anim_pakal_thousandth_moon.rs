//! Anim Pakal, Thousandth Moon — `{1}{R}{W}` 1/2 Legendary red/white Human Soldier.
//! "Whenever you attack with one or more non-Gnome creatures, put a +1/+1 counter on
//!  Anim Pakal, then create X 1/1 colorless Gnome artifact creature tokens that are
//!  tapped and attacking, where X is the number of +1/+1 counters on Anim Pakal."
//!
//! GAP: X = number of +1/+1 counters on Anim Pakal — querying counters on a specific
//! object is not available via the script:: helpers (only count_matching on the
//! battlefield is available). Also, creating tokens that enter tapped and attacking
//! is not modeled (CreateToken puts tokens onto the battlefield normally).

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
    let name = reg.interner_mut().intern("Anim Pakal, Thousandth Moon");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    // Pre-intern Gnome for potential future use
    let _gnome = reg.interner_mut().intern("Gnome");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger condition should be "whenever you attack with one or more
                // non-Gnome creatures"; CreatureAttacks fires per-creature but lacks
                // a "non-subtype" filter. Using CreatureAttacks with You as an approximation.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: attack_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Put a +1/+1 counter on Anim Pakal (trig.source).
    // GAP: X = number of +1/+1 counters on Anim Pakal — script helpers cannot
    // query counters on a specific object. Token creation (X Gnome tokens tapped
    // and attacking) is also not expressible.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
