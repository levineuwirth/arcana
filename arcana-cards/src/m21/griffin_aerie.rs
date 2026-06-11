//! Griffin Aerie — `{1}{W}` enchantment.
//! "At the beginning of your end step, if you gained 3 or more life this
//! turn, create a 2/2 white Griffin creature token with flying."
//!
//! // GAP: intervening-if — "if you gained 3 or more life this turn" has
//! // no `conditions::`/`script::` accessor (the per-turn counters cover
//! // draws/discards/spells only); the trigger fires unconditionally.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Griffin Aerie");
    let _griffin = reg.interner_mut().intern("Griffin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "if you gained 3 or more life this
                // turn" — no life-gained-this-turn accessor.
                intervening_if: None,
                effect: spawn_griffin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 2/2 white Griffin creature token with flying."
fn spawn_griffin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let griffin = reg.interner().lookup("Griffin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: griffin,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
