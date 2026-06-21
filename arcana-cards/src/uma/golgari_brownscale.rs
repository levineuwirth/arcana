//! Golgari Brownscale — `{1}{G}{G}` 2/3 green Lizard.
//!
//! * When this card is put into your hand from your graveyard, you
//!   gain 2 life.
//! * Dredge 2. (No KeywordAbility::Dredge variant — GAP.)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Golgari Brownscale");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Dredge 2 — no KeywordAbility::Dredge variant available.

    // "When this card is put into your hand from your graveyard" — a self
    // ZoneChange from Graveyard to Hand; filter restricted to this card by name.
    let self_filter = ObjectFilter {
        name: reg.interner().lookup("Golgari Brownscale"),
        ..ObjectFilter::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: self_filter,
                from: Some(Zone::Graveyard(0)),
                to: Zone::Hand(0),
            },
            intervening_if: None,
            effect: gain_two_life,
            trigger_zones: vec![Zone::Graveyard(0), Zone::Hand(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_two_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 2,
    }]
}
