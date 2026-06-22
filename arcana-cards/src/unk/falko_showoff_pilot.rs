//! Falko, Showoff Pilot — `{U}{R}{W}` 3/3 Legendary Bird Pilot (U/R/W).
//!
//! Oracle:
//! * "As long as a Spacecraft or Vehicle you control dealt combat damage to an
//!    opponent this turn, cards in your hand have spectacle. Their spectacle
//!    cost is equal to their mana cost minus {2}." — GAP: a conditional static
//!    that grants the Spectacle alternative-cast mechanic to cards in hand; no
//!    API surface (alternative casting costs aren't modeled).
//! * "Whenever you cast a spell for its spectacle cost, put a +1/+1 counter on
//!    Falko." — wired (closest variant: a spell you cast). GAP: there is no
//!    "cast for its spectacle cost" detector, so the counter cannot be gated to
//!    spectacle casts; the effect body returns `Vec::new()` rather than fire on
//!    every spell.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Falko, Showoff Pilot");
    let bird = reg.interner_mut().intern("Bird");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "cards in your hand have spectacle …" — alternative-cast
    //      grant with no API surface.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: spectacle_cast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn spectacle_cast(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "cast a spell for its SPECTACLE cost" — no detector for the alternative
    //      cost used; firing on every spell would be materially wrong.
    Vec::new()
}
