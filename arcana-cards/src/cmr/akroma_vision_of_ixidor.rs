//! Akroma, Vision of Ixidor — `{5}{W}{W}` 6/6 Legendary Angel.
//! "Flying, first strike, vigilance, trample
//!  At the beginning of each combat, until end of turn, each other creature
//!  you control gets +1/+1 if it has flying, +1/+1 if it has first strike,
//!  and so on for double strike, deathtouch, haste, hexproof,
//!  indestructible, lifelink, menace, protection, reach, trample,
//!  vigilance, and partner.
//!  Partner"
//!
//! Flying / First strike / Vigilance / Trample are keywords. Partner is not
//! an expressible KeywordAbility, so it is omitted. The begin-combat trigger
//! is wired, but the per-keyword conditional pump of each other creature has
//! no demonstrated keyword-filtered cumulative-pump hook, so the effect is
//! GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akroma, Vision of Ixidor");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Partner is not an expressible KeywordAbility.
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: each_combat_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_combat_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each other creature you control gets +1/+1 for each of the listed
    // keywords it has" — no keyword-filtered cumulative-pump hook for a
    // per-keyword stacking bonus on this shape.
    Vec::new()
}
