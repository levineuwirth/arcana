//! Broodspinner — `{B}{G}` 2/3 Spider.
//! Reach.
//! When this creature enters, surveil 2.
//! {4}{B}{G}, {T}, Sacrifice this creature: Create a number of 1/1 black and
//! green Insect creature tokens with flying equal to the number of card types
//! among cards in your graveyard.
//!
//! Reach is a base keyword and the ETB surveil is implemented. The activated
//! ability is GAP'd: its token count scales with "the number of card types
//! among cards in your graveyard", and no script:: helper computes distinct
//! card types in a graveyard, so the dynamic count is inexpressible.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Broodspinner");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: activated ability "{4}{B}{G}, {T}, Sacrifice this: create N 1/1
    //      Insect flyers where N = card types in your graveyard" — no helper
    //      for distinct card-type count.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_surveil,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_surveil(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil { player: trig.controller, count: 2 }]
}
