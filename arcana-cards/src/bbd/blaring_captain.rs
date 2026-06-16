//! Blaring Captain — `{3}{B}` 2/2 Azra Warrior.
//!
//! Oracle text:
//! * Partner with Blaring Recruiter (ETB tutor-to-hand for the partner).
//! * Whenever this creature attacks, attacking Warriors get +1/+1 until end of
//!   turn.
//!
//! Partner / Partner with are not usable `KeywordAbility` variants and the
//! partner ETB ("target player may put Blaring Recruiter into their hand …")
//! is not expressible, so both are GAP'd. The attack trigger fires correctly
//! via `SelfAttacks`, but its effect ("attacking Warriors") needs an
//! attacking-creature filter that the supported `ObjectFilter` refinements do
//! not provide, so the effect body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blaring Captain");
    let azra = reg.interner_mut().intern("Azra");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(azra);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Partner with Blaring Recruiter / Partner — not usable
        // KeywordAbility variants, and the partner-tutor ETB is unexpressible.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: pump_attacking_warriors,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_attacking_warriors(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "attacking Warriors get +1/+1" requires an attacking-creature filter
    // (combat-status restriction) not available in the supported ObjectFilter
    // refinements; pumping all Warriors would overcount non-attacking ones.
    Vec::new()
}
