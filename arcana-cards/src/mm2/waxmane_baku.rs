//! Waxmane Baku — `{2}{W}` 2/2 Spirit.
//! "Whenever you cast a Spirit or Arcane spell, you may put a ki counter on this
//! creature."
//! `{1}, Remove X ki counters from this creature: Tap X target creatures.`
//!
//! The cast trigger is wired (SpellCast filtered to Spirit/Arcane spells you cast)
//! and adds a ki counter to the source; the "you may" optionality is GAP'd. The
//! activated ability removes X ki counters to tap X target creatures — both the
//! X-valued counter-removal cost and the X dynamic targets are not expressible, so
//! the whole activated ability is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waxmane Baku");
    let spirit = reg.interner_mut().intern("Spirit");
    let _ki = reg.interner_mut().intern("ki");
    let arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let spell_filter = ObjectFilter::new()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![spirit, arcane]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "{1}, Remove X ki counters from this creature: Tap X target creatures."
    // — X-valued counter-removal cost and X dynamic targets are not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_ki,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_ki(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may" optionality on placing the ki counter.
    let ki = match reg.interner().lookup("ki") {
        Some(s) => CounterKind::Named(s),
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: ki,
        count: 1,
    }]
}
