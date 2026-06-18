//! Slaying Mantis — `{5}{G}{G}` 6/6 Creature — Insect Wrestler (Unfinity).
//!
//! An Un-set (acorn) card whose abilities rely on physical-dexterity
//! mechanics that the engine does not model:
//! * "Just a second" — a stack-state restriction tied to physically
//!   tossing the card; not expressible.  // GAP
//! * "This creature enters by being thrown from a distance of at least
//!   three feet." — physical-throw entry; not expressible.  // GAP
//! * "When this creature enters, it fights each creature an opponent
//!   controls that it touched as it entered." — the set of "touched"
//!   creatures is determined by the physical throw, which is unmodelable,
//!   so the fight targets cannot be computed.  // GAP
//!
//! `Fight` is not a `KeywordAbility` variant, so `keywords` is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slaying Mantis");
    let insect = reg.interner_mut().intern("Insect");
    let wrestler = reg.interner_mut().intern("Wrestler");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(wrestler);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: "Fight" is an Un-set keyword with no KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_fight_touched,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_fight_touched(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "fights each creature an opponent controls that it touched as it
    // entered" — the touched set is determined by a physical throw and cannot
    // be computed by the engine.
    Vec::new()
}
