//! Hurkyl, Master Wizard — `{1}{U}{U}` 2/4 Legendary blue Human Wizard Advisor.
//! "At the beginning of your end step, if you've cast a noncreature spell this
//! turn, reveal the top five cards of your library. For each card type among
//! noncreature spells you've cast this turn, you may put a card of that type
//! from among the revealed cards into your hand. Put the rest on the bottom of
//! your library in a random order."
//! GAP: intervening_if — "if you've cast a noncreature spell this turn" not
//! expressible as an intervening_if condition.
//! GAP: effect — "for each card type among noncreature spells you've cast this
//! turn, you may put a card of that type" requires multi-pick by card type
//! among a revealed set; DigTopN supports only single-pick. Best effort: look
//! at top 5, put one noncreature card into hand, rest to bottom random.

use arcana_core::effects::{DigRest, Effect};
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
use arcana_core::targets::ControllerConstraint;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hurkyl, Master Wizard");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening_if — "if you've cast a noncreature spell this
                // turn" is not expressible as an intervening_if condition.
                intervening_if: None,
                effect: end_step_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "for each card type among noncreature spells you've cast
    // this turn, you may put a card of that type" requires multi-pick by card
    // type among a revealed set; not expressible with DigTopN (single-pick
    // only). Best effort: look at top 5, put one noncreature card into hand.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
        rest: DigRest::BottomRandom,
    }]
}
