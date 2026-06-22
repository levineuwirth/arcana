//! Ureni of the Unwritten — `{4}{G}{U}{R}` 7/7 Legendary Spirit Dragon (G/U/R).
//! Flying, trample.
//! Whenever Ureni enters or attacks, look at the top eight cards of your
//!   library. You may put a Dragon creature card from among them onto the
//!   battlefield. Put the rest on the bottom of your library in a random order.

use arcana_core::effects::{Effect, KeywordAbility, DigRest, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ureni of the Unwritten");
    let spirit = reg.interner_mut().intern("Spirit");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "When Ureni enters, …"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: dig_for_dragon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "… or attacks, …" (no combined enters-or-attacks trigger; two defs.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: dig_for_dragon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dig_for_dragon(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Approximated with reveal-until: reveal the top (up to eight) cards
    // until a Dragon creature card and put it onto the battlefield, rest
    // on the bottom in random order. (DigTopN only goes to hand; the
    // optional "you may" vs deterministic first-match is a fidelity gap.)
    let filter = script::subtype_filter(reg, "Dragon");
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter,
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(8),
    }]
}
