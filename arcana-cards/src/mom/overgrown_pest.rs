//! Overgrown Pest — `{2}{G}` 2/2 green Creature — Pest.
//! When this creature enters, look at the top five cards of your library. You may reveal a land or
//! double-faced card from among them and put that card into your hand. Put the rest on the bottom.
//! GAP: the takeable set is a "land OR double-faced" DISJUNCTION. The double_faced() ObjectFilter
//!      predicate now exists, but ObjectFilter ANDs its predicates, so "land OR double-faced" can't
//!      be a single filter and DigTopN takes only one filter. Wired faithfully as a DigTopN over
//!      the top 5 (rest to bottom) with the LAND half of the disjunction; the double-faced half is
//!      the remaining blocker (a DigTopN that accepts a disjunction / OR of filters).

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Overgrown Pest");
    let pest_sub = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::SelfEntersBattlefield, intervening_if: None, effect: etb_look, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() }))
}

fn etb_look(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // "Look at the top 5, you may take a land OR double-faced card to hand, rest on the bottom."
    // DigTopN(top 5, optional pick to hand, rest to bottom) is the exact structure. The takeable
    // filter is a land-OR-double-faced DISJUNCTION, which a single ANDing ObjectFilter / DigTopN
    // can't express, so only the LAND half is wired here; the double-faced half is the named GAP.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(ObjectFilter::new().with_types(TypeLine::LAND.into())),
        rest: DigRest::BottomRandom,
    }]
}
