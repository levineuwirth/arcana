//! Blossom Prancer — `{3}{G}{G}` 4/4 Creature — Spirit.
//! Reach.
//! When this creature enters, look at the top five cards of your
//! library. You may reveal a creature or enchantment card from among
//! them and put it into your hand. Put the rest on the bottom of your
//! library in a random order. If you didn't put a card into your hand
//! this way, you gain 4 life. (The dig-and-take is modeled via DigTopN;
//! the "if you didn't take a card, gain 4 life" rider is GAP'd — DigTopN
//! doesn't report whether a card was taken.)

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blossom Prancer");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_dig(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if you didn't put a card into your hand this way, you gain 4
    // life" — DigTopN does not report whether a card was taken.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(
            ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ENCHANTMENT)),
        ),
        rest: DigRest::BottomRandom,
    }]
}
