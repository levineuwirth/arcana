//! Glissa's Retriever — `{5}{G}` 6/6 Phyrexian Beast with Haste and Toxic 3.
//! "This creature can't be blocked by creatures with power 2 or less.
//!  Corrupted — When this creature dies, exile it. When you do, return up to X
//!  target cards from your graveyard to your hand, where X is the number of
//!  opponents who have three or more poison counters."

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Glissa's Retriever");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Haste, KeywordAbility::Toxic(3)],
        ..Default::default()
    };

    // GAP: static "can't be blocked by creatures with power 2 or less" — a
    // power-filtered evasion static is not expressible from the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile it. When you do, return up to X target cards from your graveyard
    // to your hand, where X is the number of opponents who have three or more
    // poison counters" — a reflexive "when you do" trigger with X derived from a
    // per-opponent poison-counter count is not expressible.
    Vec::new()
}
