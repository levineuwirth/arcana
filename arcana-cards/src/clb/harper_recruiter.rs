//! Harper Recruiter — `{2}{W}` 3/1 Human Warrior with Flying.
//! "Whenever this creature attacks, look at the top four cards of your library.
//!  You may reveal a Cleric card, a Rogue card, a Warrior card, and/or a Wizard
//!  card from among them and put those cards into your hand. Put the rest on the
//!  bottom of your library in a random order."
//!
//! Modeled with `DigTopN` (look at top 4, put one chosen matching card into
//! hand, rest on bottom in random order). The filter restricts the take to a
//! Cleric/Rogue/Warrior/Wizard card via subtype-OR. GAP: DigTopN takes only
//! ONE card; the "and/or" multi-take (up to four) is not expressible, so this
//! is the single-card approximation.

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
    let name = reg.interner_mut().intern("Harper Recruiter");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let _cleric = reg.interner_mut().intern("Cleric");
    let _rogue = reg.interner_mut().intern("Rogue");
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack_dig(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let syms: Vec<_> = ["Cleric", "Rogue", "Warrior", "Wizard"]
        .iter()
        .filter_map(|n| reg.interner().lookup(n))
        .collect();
    let filter = ObjectFilter::default().with_subtypes_any(syms);
    // GAP: DigTopN takes only ONE card; the "and/or" multi-take (up to four
    // matching cards) is not expressible — this takes a single matching card.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
