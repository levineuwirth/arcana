//! Spelling Bee — `{2}{G}` 1/1 green Alien Insect.
//!
//! Flying, deathtouch.
//! Whenever this creature deals combat damage to a player, that player
//! looks at the top card of your library and chooses a word on that
//! card. You spell that word; if correct, draw a card, otherwise scry 1.
//! (The spelling minigame is not modeled — see GAP below.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spelling Bee");
    let alien = reg.interner_mut().intern("Alien");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // Scryfall lists Scry as a keyword but it is a resolution-time
        // action, not a static/listed keyword on this card.
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever this creature deals combat damage to a player, that
            // player looks at the top card of your library and chooses a
            // word; you spell it; if correct draw, otherwise scry 1."
            // GAP: the word-spelling minigame (looking at a card, choosing a
            // word, spelling it, branching on success) has no engine
            // representation — emit the trigger shape with an empty effect.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: spelling_minigame,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn spelling_minigame(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: spelling-word minigame not modeled.
    Vec::new()
}
