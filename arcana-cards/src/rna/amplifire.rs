//! Amplifire — `{2}{R}{R}` 1/1 red Elemental. "At the beginning of your
//! upkeep, reveal cards from the top of your library until you reveal a
//! creature card. Until your next turn, this creature's base power
//! becomes twice that card's power and its base toughness becomes twice
//! that card's toughness. Put the revealed cards on the bottom of your
//! library in a random order."

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::ControllerConstraint;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Amplifire");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: reveal_until_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn reveal_until_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The reveal-until-a-creature dig + bottom-the-rest is expressible.
    // GAP: "this creature's base power/toughness becomes TWICE the
    // revealed card's power/toughness until your next turn" — there is
    // no effect that reads the revealed card's P/T and applies a
    // dynamic SetBasePT (RevealUntil does not surface the found card's
    // characteristics to a follow-on effect). Emitting only the reveal.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        found_dest: RevealDest::Hand,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}
