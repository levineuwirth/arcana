//! Eclipsed Kithkin — `{G/W}{G/W}` 2/1 green/white Kithkin Scout.
//! "When this creature enters, look at the top four cards of your library.
//! You may reveal a Kithkin, Forest, or Plains card from among them and put
//! it into your hand. Put the rest on the bottom of your library in a
//! random order."
//! GAP: DigTopN supports a single ObjectFilter pick. The oracle allows
//! picking a card matching one of three disjoint criteria (Kithkin subtype,
//! Forest subtype, or Plains subtype). The engine's ObjectFilter supports
//! with_subtypes_any for multiple subtypes, so we approximate by treating
//! "Forest or Plains" as land subtypes alongside "Kithkin" creature subtype.
//! This is a best-effort approximation using DigTopN with a combined filter.

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
    let name = reg.interner_mut().intern("Eclipsed Kithkin");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let scout = reg.interner_mut().intern("Scout");
    // Pre-intern subtypes needed for the filter at resolve time.
    let _forest = reg.interner_mut().intern("Forest");
    let _plains = reg.interner_mut().intern("Plains");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_four(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Build a filter for Kithkin, Forest, or Plains cards.
    // with_subtypes_any matches cards that have any of the listed subtypes.
    let kithkin = reg.interner().lookup("Kithkin");
    let forest = reg.interner().lookup("Forest");
    let plains = reg.interner().lookup("Plains");
    let mut subtype_ids = Vec::new();
    if let Some(s) = kithkin { subtype_ids.push(s); }
    if let Some(s) = forest { subtype_ids.push(s); }
    if let Some(s) = plains { subtype_ids.push(s); }
    let filter = if subtype_ids.is_empty() {
        None
    } else {
        Some(ObjectFilter::default().with_subtypes_any(subtype_ids))
    };
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter,
        rest: DigRest::BottomRandom,
    }]
}
