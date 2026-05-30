//! Magic Designer — `{2}` 2/2 colorless Human Gamer.
//! "When Magic Designer enters the battlefield, look at the top 10 cards of your library.
//! You may reveal a playtest card from among them and put it into your hand, then put the
//! rest on the bottom of your library in a random order."
//! GAP: "playtest card" subtype filter — the engine has no subtype for playtest cards;
//! we model this as DigTopN with filter: None (any card) as the closest approximation.

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Magic Designer");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_top_ten,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_top_ten(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "playtest card" subtype filter — no subtype for playtest cards in engine;
    // approximated as any card (filter: None).
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 10,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
