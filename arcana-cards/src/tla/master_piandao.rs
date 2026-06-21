//! Master Piandao — `{4}{W}` Legendary 4/4 Human Warrior Ally with First
//! strike.
//! "Whenever Master Piandao attacks, look at the top four cards of your
//! library. You may reveal an Ally, Equipment, or Lesson card from among
//! them and put it into your hand. Put the rest on the bottom of your
//! library in a random order." (DigTopN, subtype-OR filter).

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Master Piandao");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let ally = reg.interner_mut().intern("Ally");
    // Pre-intern the dig-filter subtypes so the resolver can recover them.
    let _equipment = reg.interner_mut().intern("Equipment");
    let _lesson = reg.interner_mut().intern("Lesson");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: dig_for_ally,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dig_for_ally(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let syms: Vec<_> = ["Ally", "Equipment", "Lesson"]
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: Some(ObjectFilter::new().with_subtypes_any(syms)),
        rest: DigRest::BottomRandom,
    }]
}
