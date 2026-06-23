//! Keleth, Sunmane Familiar — `{1}{W}` 1/1 Legendary Horse.
//! "Whenever a commander you control attacks, put a +1/+1 counter on it."
//! "Partner"
//!
//! GAP: keyword Partner — not in the usable KeywordAbility surface
//!      (the "two commanders" deckbuilding rule is unmodeled).
//! GAP: the attack trigger is restricted to "a commander you control";
//!      there is no commander filter on ObjectFilter, so the restriction
//!      is approximated to "a creature you control" — the +1/+1 counter
//!      is placed on the attacking creature via trig.attacking_creature().

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keleth, Sunmane Familiar");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
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
