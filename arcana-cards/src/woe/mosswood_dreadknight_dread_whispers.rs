//! Mosswood Dreadknight // Dread Whispers — `{1}{G}` Human Knight creature 3/2
//! with the Adventure face "Dread Whispers" (`{1}{B}` sorcery, "You draw a
//! card and you lose 1 life").
//!
//! # Creature face
//!
//! * Trample
//! * "When this creature dies, you may cast it from your graveyard as an
//!   Adventure until the end of your next turn." — GAP: "cast from graveyard
//!   as Adventure" grants a transient cast permission from graveyard; this is
//!   not expressible in the Effect catalog. The triggered ability is registered
//!   with no effect.
//!
//! # Adventure face
//!
//! "You draw a card and you lose 1 life." — fully expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mosswood Dreadknight");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid creature cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Dread Whispers");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid adventure cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "You draw a card and you lose 1 life.".into(),
        target_requirements: vec![],
        modal: None,
        effect: dread_whispers_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure)
    )
}

fn on_dies(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may cast this card from your graveyard as an Adventure until
    // the end of your next turn" — transient cast-from-graveyard permission
    // not expressible in the Effect catalog.
    Vec::new()
}

fn dread_whispers_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::LoseLife { player: entry.controller, amount: 1 },
    ]
}
