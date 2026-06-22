//! Karumonix, the Rat King — `{1}{B}{B}` 3/3 Legendary Phyrexian Rat.
//! "Toxic 1. Other Rats you control have toxic 1. When Karumonix enters, look
//!  at the top five cards of your library. You may reveal any number of Rat
//!  cards from among them and put the revealed cards into your hand. Put the
//!  rest on the bottom of your library in a random order."
//!
//! Toxic 1 is a base keyword. The "Other Rats you control have toxic 1" line is
//! a static keyword-granting ability — GAP'd (no expressible static-grant
//! primitive in this card class). The ETB dig is modeled with DigTopN over a
//! Rat filter; DigTopN is single-take only, so "reveal ANY NUMBER of Rat cards"
//! is a documented fidelity gap (the player takes at most one Rat).

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karumonix, the Rat King");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(rat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Toxic(1)],
        ..Default::default()
    };

    // GAP: "Other Rats you control have toxic 1." — static keyword-granting
    // continuous ability, not a triggered/activated ability; not expressible
    // in this card class.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_dig_for_rats,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_dig_for_rats(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity gap: DigTopN takes at most one card; the oracle allows any
    // number of Rats. Best-effort: pick one Rat card into hand, rest to bottom.
    let rat_filter = script::subtype_filter(reg, "Rat");
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(rat_filter),
        rest: DigRest::BottomRandom,
    }]
}
