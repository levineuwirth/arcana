//! Angelic Accord — `{3}{W}` enchantment (Magic 2014).
//! "At the beginning of each end step, if you gained 4 or more life this
//! turn, create a 4/4 white Angel creature token with flying."
//!
//! An each-end-step trigger. GAP: the intervening-if "if you gained 4 or
//! more life this turn" — no available `conditions::` predicate reads
//! life-gained-this-turn, so the gate is omitted (over-fires; the token is
//! created every end step).

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
    let name = reg.interner_mut().intern("Angelic Accord");
    let _angel = reg.interner_mut().intern("Angel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
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
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if "if you gained 4 or more life this
                // turn" — no conditions:: predicate exposes life gained
                // this turn; gate omitted (trigger over-fires).
                intervening_if: None,
                effect: make_angel,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 4/4 white Angel creature token with flying."
fn make_angel(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let angel = reg.interner().lookup("Angel").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: angel,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
