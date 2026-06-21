//! Resplendent Marshal — `{1}{W}{W}` 3/3 Angel Warrior with Flying.
//! When this creature enters or dies, you may exile another creature card from
//! your graveyard. When you do, put a +1/+1 counter on each creature you control
//! other than this creature that shares a creature type with the exiled card.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Resplendent Marshal");
    let angel = reg.interner_mut().intern("Angel");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_and_share_type,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: exile_and_share_type,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exile_and_share_type(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile a creature card from your graveyard; then put a +1/+1 counter on
    // each creature you control that shares a creature type with the exiled card"
    // — needs a reflexive trigger plus a filter dynamically derived from the
    // exiled card's subtypes; not expressible. The whole effect is omitted.
    Vec::new()
}
