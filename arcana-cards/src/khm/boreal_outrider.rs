//! Boreal Outrider — `{2}{G}` 3/2 green Snow Elf Warrior.
//! "Whenever you cast a creature spell, if {S} of any of that spell's colors was
//! spent to cast it, that creature enters with an additional +1/+1 counter on it."
//! GAP: Snow-mana-spent check ("if {S} was spent") — no snow-mana payment filter
//! in ObjectFilter or SpellCast condition; trigger fires unconditionally for any
//! creature spell you cast (over-permissive).

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
    let name = reg.interner_mut().intern("Boreal Outrider");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::creature()),
                    caster: ControllerConstraint::You,
                },
                // GAP: "if {S} was spent to cast it" — no snow-mana payment
                // predicate available; trigger fires for all creature spells.
                intervening_if: None,
                effect: spell_cast_add_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn spell_cast_add_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: The entering object (the cast creature) needs AddCounters on entry.
    // trig.entering_object() is not available for SpellCast triggers; using
    // trig.source as a placeholder — this will not correctly target the cast spell.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
