//! Marauding Raptor — `{1}{R}` 2/3 red Dinosaur.
//! Creature spells you cast cost {1} less to cast.
//! Whenever another creature you control enters, this creature deals 2 damage
//! to it. If a Dinosaur is dealt damage this way, this creature gets +2/+0
//! until end of turn.
//!
//! Abilities:
//!  - "Creature spells you cast cost {1} less to cast." — static cost-reduction;
//!    no expressible primitive for this card class → GAP'd.
//!  - Triggered (another creature you control enters): deal 2 damage to the
//!    entering creature. The conditional "If a Dinosaur is dealt damage this
//!    way, this creature gets +2/+0" rider depends on inspecting whether the
//!    damaged object was a Dinosaur AFTER the damage is applied (post-resolution
//!    state not available to the resolver) → the pump rider is GAP'd; the
//!    2 damage is emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "Creature spells you cast cost {1} less to cast." — static
// cost-reduction; no expressible primitive for this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marauding Raptor");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "another creature you control enters" → enters-under-your-control,
            // watcher is not the entering object (use AnotherMatching so the
            // source's own ETB does not re-fire on itself).
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: damage_entering_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn damage_entering_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "another creature you control" — ignore our own ETB.
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    if id == trig.source {
        return Vec::new();
    }
    // GAP: "If a Dinosaur is dealt damage this way, this creature gets +2/+0
    // until end of turn." — the rider must inspect, after the damage resolves,
    // whether the damaged object was a Dinosaur; that post-damage state is not
    // available to this resolver, so the pump is dropped.
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(id),
        amount: 2,
    }]
}
