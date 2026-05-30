//! Traveling Botanist — `{1}{G}` 2/3 green Dog Scout.
//! "Whenever this creature becomes tapped, look at the top card of your
//! library. If it's a land card, you may reveal it and put it into your
//! hand. If you don't put the card into your hand, you may put it into
//! your graveyard."
//!
//! Modeled as DigTopN { count: 1, filter: land, rest: Graveyard }.
//! The "may put into graveyard" rider for non-land cards is partially
//! approximated: DigRest::Graveyard puts unchosen cards into the
//! graveyard (the "may" nuance — putting back on top vs. graveyard —
//! is a GAP: engine always sends unchosen cards to graveyard rather
//! than offering a choice between graveyard and top of library).

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Traveling Botanist");
    let dog = reg.interner_mut().intern("Dog");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: look_top_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn look_top_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::LAND.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::Graveyard,
    }]
}
