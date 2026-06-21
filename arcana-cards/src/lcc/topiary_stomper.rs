//! Topiary Stomper — `{1}{G}{G}` 4/4 Creature — Plant Dinosaur with Vigilance.
//!
//! Oracle:
//! * Vigilance — base keyword.
//! * When this creature enters, search your library for a basic land card, put
//!   it onto the battlefield tapped, then shuffle. — an ETB tutor; modeled with
//!   `TutorToBattlefield` (basic-land filter, tapped). Shuffle is automatic.
//! * This creature can't attack or block unless you control seven or more
//!   lands. — a STATIC attack/block restriction; not a triggered/activated
//!   ability and not expressible here. GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Topiary Stomper");
    let plant = reg.interner_mut().intern("Plant");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static — "This creature can't attack or block unless you control
    // seven or more lands" (conditional attack/block restriction).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_basic_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: search your library for a basic land card, put it onto the battlefield
/// tapped, then shuffle (shuffle is automatic).
fn etb_tutor_basic_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
        tapped: true,
    }]
}
