//! Jaded Sell-Sword — `{3}{R}` 4/3 red Dragon Warrior.
//! "When this creature enters, if mana from a Treasure was spent to cast it, it gains first
//! strike and haste until end of turn."
//! GAP: "if mana from a Treasure was spent to cast it" — mana source tracking not available;
//! intervening-if not modeled (None); applying keywords unconditionally.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Jaded Sell-Sword");
    let dragon = reg.interner_mut().intern("Dragon");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if mana from a Treasure was spent to cast it" not expressible as intervening-if
                intervening_if: None,
                effect: etb_first_strike_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_first_strike_haste(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mana source tracking not available; applying keywords unconditionally
    vec![Effect::Pump {
        target: trig.source,
        power: 0,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
    }]
}
