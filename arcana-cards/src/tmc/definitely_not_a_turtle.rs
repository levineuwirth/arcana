//! Definitely Not a Turtle — `{3}` 3/2 Creature — Mutant Ninja Turtle.
//! When this creature dies, look at the top six cards of your library. You may reveal a land
//! or legendary Turtle card from among them and put it into your hand. Put the rest on the
//! bottom of your library in a random order.
//! Uses DigTopN with a filter for land-or-legendary-Turtle (land type via types filter;
//! legendary Turtle via supertypes+subtype filter — DigTopN single-take is the best match).
//! GAP: DigTopN takes a single ObjectFilter; the "land OR legendary Turtle" disjunction
//! requires two separate filter conditions which cannot be OR'd in one ObjectFilter.
//! Best-effort: filter for land cards only (the more common find); legendary Turtle is GAP.

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
    let name = reg.interner_mut().intern("Definitely Not a Turtle");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
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
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "land or legendary Turtle" is a disjunction of two ObjectFilter conditions;
    // DigTopN accepts a single filter. Modeling the land half only as best-effort.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::LAND.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
