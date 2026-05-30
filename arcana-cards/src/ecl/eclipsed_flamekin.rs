//! Eclipsed Flamekin — `{1}{U/R}{U/R}` 1/4 blue/red Elemental Scout.
//! "When this creature enters, look at the top four cards of your library.
//! You may reveal an Elemental, Island, or Mountain card from among them
//! and put it into your hand. Put the rest on the bottom of your library
//! in a random order."
//! Uses Effect::DigTopN. The filter ("Elemental, Island, or Mountain card")
//! requires a subtype/land-type OR that cannot be expressed as a static fn
//! pointer (would need a closure over the interner). GAP: the card-type
//! filter is dropped; any card may be taken (approximation).

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Eclipsed Flamekin");
    let elemental = reg.interner_mut().intern("Elemental");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: dig_top_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dig_top_four(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: filter should restrict to "Elemental, Island, or Mountain card"
    // (subtype-or with land subtypes); cannot express as static fn pointer —
    // would require a closure over the interner. Using filter: None as
    // approximation (any card takeable).
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
