//! Atraxa, Grand Unifier — `{3}{G}{W}{U}{B}` 7/7 Legendary Phyrexian Angel.
//! Flying, vigilance, deathtouch, lifelink.
//! "When Atraxa enters, reveal the top ten cards of your library. For each
//!  card type, you may put a card of that type from among them into your
//!  hand. Put the rest on the bottom in a random order."
//!
//! The four keywords are base characteristics. The ETB ability is GAP'd:
//! "for each card type, take one matching card" is a per-type multi-pick over
//! a fixed reveal — DigTopN is single-take only, so the one-per-type harvest
//! is not faithfully expressible.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Atraxa, Grand Unifier");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Deathtouch,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: on_enter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_enter(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: reveal top ten, then for EACH card type optionally take one matching
    // card to hand (a per-type multi-pick over the same reveal), rest to bottom.
    // DigTopN is single-take and cannot harvest one card per card type.
    Vec::new()
}
