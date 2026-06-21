//! Damia, Sage of Stone — `{4}{B}{G}{U}` 4/4 Legendary Gorgon Wizard
//! with Deathtouch.
//! "Skip your draw step."
//! "At the beginning of your upkeep, if you have fewer than seven cards
//! in hand, draw cards equal to the difference."
//!
//! Deathtouch is a base keyword. "Skip your draw step" is a static
//! turn-structure modifier with no expressible primitive — GAP'd. The
//! upkeep trigger is intervening-if gated (fewer than seven cards) and
//! draws up to seven minus the current hand size.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Damia, Sage of Stone");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    // GAP (static): "Skip your draw step." Turn-structure modification has no
    // expressible primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_fewer_than_seven),
                effect: refill_to_seven,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_fewer_than_seven(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::hand_size(s, you) < 7
}

fn refill_to_seven(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let hand = script::hand_size(state, trig.controller);
    let count = 7u32.saturating_sub(hand);
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: trig.controller, count }]
}
