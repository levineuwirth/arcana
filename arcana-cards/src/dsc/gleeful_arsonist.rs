//! Gleeful Arsonist — `{2}{R}` 1/2 Human Wizard with Undying.
//!
//! * Undying — base keyword (CR 702.93): when it dies with no +1/+1
//!   counters, returns with one.
//! * "Whenever an opponent casts a noncreature spell, this creature
//!   deals damage equal to its power to that player." — a `SpellCast`
//!   trigger filtered to noncreature spells cast by an opponent; the
//!   damage amount is its power (dynamic) and the target is the casting
//!   player (`trig.triggering_caster`).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Gleeful Arsonist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Undying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: deal_power_to_caster,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn deal_power_to_caster(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(player) = trig.triggering_caster() else {
        return Vec::new();
    };
    let amount = script::power_of(state, trig.source).max(0) as u32;
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(player),
        amount,
    }]
}
