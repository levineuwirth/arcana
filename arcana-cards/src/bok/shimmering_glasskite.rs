//! Shimmering Glasskite — `{3}{U}` 2/3 Creature — Spirit. U.
//! Flying.
//! "Whenever this creature becomes the target of a spell or ability for the
//! first time each turn, counter that spell or ability." — SelfBecomesTarget
//! (any caster), frequency OncePerTurn for "first time each turn". GAP: the
//! "counter that spell or ability" body — no PendingTrigger accessor exposes
//! the targeting spell/ability's stack id, so Effect::Counter can't be
//! pointed at it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shimmering Glasskite");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTarget {
                caster: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: counter_targeting,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_targeting(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no accessor exposes the targeting spell/ability's stack id, so
    // Effect::Counter cannot target it.
    Vec::new()
}
