//! Gríma, Saruman's Footman — `{2}{U}{B}` 1/4 Legendary Creature — Human
//! Advisor (blue/black).
//!
//! * "Gríma can't be blocked." — modeled as a SelfEnters trigger applying
//!   CantBeBlocked to itself for as long as it is on the battlefield.
//! * "Whenever Gríma deals combat damage to a player, that player exiles
//!   cards from the top of their library until they exile an instant or
//!   sorcery card. You may cast that card without paying its mana cost. ..."
//!   — there is no "exile from an opponent's library and let YOU cast it for
//!   free" primitive (RevealUntil moves cards within that player's own zones;
//!   it cannot grant the controller a free cast). GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Gríma, Saruman's Footman");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: combat-damage "exile from their library, you may cast for free"
    // has no expressible primitive.
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfEntersBattlefield,
        intervening_if: None,
        effect: cant_be_blocked,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }))
}

fn cant_be_blocked(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}
