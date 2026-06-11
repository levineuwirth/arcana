//! Chalk Outline — `{3}{G}` enchantment.
//! "Whenever one or more creature cards leave your graveyard, create a
//! 2/2 white and blue Detective creature token, then investigate."
//!
//! // GAP: trigger — "leave your graveyard" (to ANY destination) cannot
//! // be expressed: `ZoneChange.to` requires a single zone. The closest
//! // wiring (graveyard → battlefield, the most common exit) is used and
//! // under-fires on exits to hand/library/exile. "One or more …
//! // [batch]" also fires once per card here, not once per batch.

use arcana_core::effects::{CommodityToken, Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Chalk Outline");
    let _detective = reg.interner_mut().intern("Detective");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: should fire on creature cards leaving your
                // graveyard to ANY zone; `to` must be a single zone, so
                // graveyard → battlefield is wired.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: detective_and_clue,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 2/2 white and blue Detective creature token, then
/// investigate."
fn detective_and_clue(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let detective = reg.interner().lookup("Detective").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(detective);
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: detective,
                colors: ColorSet::white() | ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Clue,
            count: 1,
        },
    ]
}
