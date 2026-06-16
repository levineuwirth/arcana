//! Brightfield Mustang — `{3}{W}` 3/3 Horse Mount.
//! Whenever this creature attacks while saddled, untap it and put a
//! +1/+1 counter on it. The "while saddled" gate is GAP (no saddled
//! board-state condition predicate), so the trigger is wired on
//! SelfAttacks with `intervening_if: None` and the untap + counter is
//! emitted. Saddle 1 is GAP (not in the usable keyword surface and the
//! "tap any number of creatures with total power N" cost is not an
//! expressible ActivationCost field).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brightfield Mustang");
    let horse = reg.interner_mut().intern("Horse");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Saddle 1 — not in the usable keyword surface; the
    // "tap any number of other creatures with total power 1 or more"
    // activation cost is not an expressible ActivationCost field.
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "while saddled" — no saddled board-state condition; the
            // trigger fires on every attack.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_untap_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_untap_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Untap { target: trig.source },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
