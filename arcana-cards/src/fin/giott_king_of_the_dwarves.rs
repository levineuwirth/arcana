//! Giott, King of the Dwarves — `{R}{W}` 1/1 Legendary Dwarf Noble.
//! Double strike.
//! Whenever Giott or another Dwarf you control enters and whenever an
//! Equipment you control enters, you may discard a card. If you do, draw a
//! card.
//!
//! Double strike is a base keyword. The compound ETB trigger is split into a
//! Dwarf-enters trigger and an Equipment-enters trigger. Both effects are
//! GAP'd: "you may discard a card. If you do, draw a card" is an optional
//! discard gate, and OptionalPayment only models Mana/Life costs.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giott, King of the Dwarves");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let noble = reg.interner_mut().intern("Noble");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_subtype_sym(dwarf)
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: may_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_subtype_sym(equipment)
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: may_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn may_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard a card. If you do, draw a card" — optional
    // discard gate; OptionalPayment only models Mana/Life costs.
    Vec::new()
}
