//! Decadent Dragon // Expensive Taste — `{2}{R}{R}` red Dragon creature 4/4.
//! Flying, trample. Whenever this creature attacks, create a Treasure token.
//! Adventure face "Expensive Taste" (`{2}{B}` Instant):
//! Exile the top two cards of target opponent's library face down. You may
//! look at and play those cards for as long as they remain exiled.
//!
//! GAP: adventure "Expensive Taste" effect — "exile top two cards face down;
//! you may look at and play them for as long as they remain exiled" is not
//! expressible (no face-down exile tracking / play-from-exile-with-source-
//! restriction in current engine). The adventure spell_ability returns
//! Vec::new() with a GAP comment.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Decadent Dragon");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    // Adventure face "Expensive Taste"
    let adv_name = reg.interner_mut().intern("Expensive Taste");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Exile the top two cards of target opponent's library face down. \
               You may look at and play those cards for as long as they remain exiled.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Player,
            count: TargetCount::Exactly(1),
            controller: Some(ControllerConstraint::Opponent),
        }],
        modal: None,
        effect: expensive_taste_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            // Whenever this creature attacks, create a Treasure token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_create_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn expensive_taste_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top two cards of target opponent's library face down;
    // you may look at and play them for as long as they remain exiled" —
    // face-down exile tracking + play-from-exile-with-source-restriction
    // not expressible in current engine.
    Vec::new()
}

fn attacks_create_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
