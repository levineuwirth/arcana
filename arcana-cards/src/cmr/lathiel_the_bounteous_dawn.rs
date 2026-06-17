//! Lathiel, the Bounteous Dawn — `{2}{G}{W}` 2/2 Legendary Unicorn with Lifelink.
//! "At the beginning of each end step, if you gained life this turn, distribute up
//! to that many +1/+1 counters among any number of other target creatures."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lathiel, the Bounteous Dawn");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            // GAP: intervening-if "if you gained life this turn" — no conditions::
            // predicate for life-gained-this-turn.
            intervening_if: None,
            effect: distribute_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn distribute_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "distribute up to X +1/+1 counters among any number of other target
    // creatures, where X = life gained this turn" — no script:: helper yields
    // life-gained-this-turn, and there is no distribute-counters Effect.
    Vec::new()
}
