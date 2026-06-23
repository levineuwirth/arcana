//! Kaalia, Zenith Seeker — `{R}{W}{B}` 3/3 Legendary Human Cleric
//! (B/R/W). Flying, vigilance.
//!
//! "When Kaalia enters, look at the top six cards of your library. You
//! may reveal an Angel card, a Demon card, and/or a Dragon card from
//! among them and put them into your hand. Put the rest on the bottom
//! of your library in a random order."
//!
//! Modeled as `Effect::DigTopN` over the top six with a subtype-OR
//! filter (Angel / Demon / Dragon) and `DigRest::BottomRandom`.
//! FIDELITY GAP: `DigTopN` takes at most ONE matching card into hand;
//! Kaalia may take up to three (one of each type). The single-take dig
//! is the closest demonstrated primitive.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaalia, Zenith Seeker");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    // Pre-intern the dig subtypes so they're in the interner for lookup.
    let _angel = reg.interner_mut().intern("Angel");
    let _demon = reg.interner_mut().intern("Demon");
    let _dragon = reg.interner_mut().intern("Dragon");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_for_fatty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_for_fatty(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let syms: Vec<_> = ["Angel", "Demon", "Dragon"]
        .iter()
        .filter_map(|s| reg.interner().lookup(*s))
        .collect();
    let filter = ObjectFilter::new().with_subtypes_any(syms);
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
