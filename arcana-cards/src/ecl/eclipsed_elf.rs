//! Eclipsed Elf — `{B/G}{B/G}{B/G}` 3/2 black/green Elf Scout.
//! "When this creature enters, look at the top four cards of your library.
//! You may reveal an Elf, Swamp, or Forest card from among them and put it
//! into your hand. Put the rest on the bottom of your library in a random
//! order."
//!
//! Implemented via DigTopN with a filter matching any of the three subtypes
//! (Elf, Swamp, Forest). DigTopN takes a single-card pick, which matches
//! the oracle's "you may reveal one … card".

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
    let name = reg.interner_mut().intern("Eclipsed Elf");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    // Pre-intern the filter subtypes for use at resolve time.
    let _swamp = reg.interner_mut().intern("Swamp");
    let _forest = reg.interner_mut().intern("Forest");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/G}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_look_top_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_look_top_four(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Build a filter matching Elf, Swamp, or Forest subtype cards.
    let elf_id = reg.interner().lookup("Elf");
    let swamp_id = reg.interner().lookup("Swamp");
    let forest_id = reg.interner().lookup("Forest");

    let mut subtype_ids = Vec::new();
    if let Some(id) = elf_id { subtype_ids.push(id); }
    if let Some(id) = swamp_id { subtype_ids.push(id); }
    if let Some(id) = forest_id { subtype_ids.push(id); }

    let filter = if subtype_ids.is_empty() {
        ObjectFilter::default()
    } else {
        ObjectFilter::new().with_subtypes_any(subtype_ids)
    };

    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
