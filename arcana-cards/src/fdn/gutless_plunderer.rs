//! Gutless Plunderer — `{2}{B}` 2/2 Creature — Skeleton Pirate with Deathtouch.
//!
//! Deathtouch.
//! Raid — When this creature enters, if you attacked this turn, look at the top
//! three cards of your library. You may put one of those cards back on top of
//! your library. Put the rest into your graveyard.
//!
//! Deathtouch is a base keyword (Raid is an ability word, not a KeywordAbility).
//! The ETB body — look at the top 3, keep one on top, the rest to graveyard —
//! is modeled with Surveil 3 (look at the top 3, put any number into your
//! graveyard and the rest back on top); keeping one and milling two is a subset
//! of Surveil 3. The "if you attacked this turn" intervening-if has no
//! conditions/script helper for a per-turn attack flag, so it is GAP'd (fires
//! unconditionally) per the no-invent rule.

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
    let name = reg.interner_mut().intern("Gutless Plunderer");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "if you attacked this turn" — no conditions/script
            // predicate for a per-turn attack flag; fires unconditionally.
            intervening_if: None,
            effect: raid_surveil,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn raid_surveil(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil { player: trig.controller, count: 3 }]
}
