//! Twilight Diviner — `{2}{B}` 3/3 black Elf Cleric.
//!
//! * "When this creature enters, surveil 2." → `SelfEntersBattlefield`
//!   trigger emitting `Effect::Surveil { count: 2 }`.
//! * "Whenever one or more other creatures you control enter, if they
//!   entered or were cast from a graveyard, create a token that's a copy
//!   of one of them. This ability triggers only once each turn." →
//!   `ZoneChange` for a creature entering under your control,
//!   `frequency: OncePerTurn`, creating a token copy of the entering
//!   creature via `Effect::CopyPermanent`. GAP: the "if they entered or
//!   were cast from a graveyard" intervening-if has no condition helper
//!   (no graveyard-origin predicate), so the trigger fires
//!   unconditionally on any creature ETB — a documented over-fire.

use arcana_core::effects::Effect;
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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Twilight Diviner");
    let elf = reg.interner_mut().intern("Elf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_surveil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                // GAP: "if they entered or were cast from a graveyard" —
                // no graveyard-origin intervening-if predicate exists.
                intervening_if: None,
                effect: copy_entering_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_surveil(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: trig.controller,
        count: 2,
    }]
}

fn copy_entering_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else { return Vec::new(); };
    // "another" — exclude this creature's own ETB.
    if id == trig.source {
        return Vec::new();
    }
    vec![Effect::CopyPermanent { target: id }]
}
