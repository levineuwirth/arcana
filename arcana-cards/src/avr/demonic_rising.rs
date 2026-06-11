//! Demonic Rising — `{3}{B}{B}` enchantment (Avacyn Restored, 2012).
//! "At the beginning of your end step, if you control exactly one creature,
//! create a 5/5 black Demon creature token with flying."
//!
//! An end-step trigger with an intervening-if expressing "exactly one
//! creature" as at-least-1 AND at-most-1.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Demonic Rising");
    let _demon = reg.interner_mut().intern("Demon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
                intervening_if: Some(if_exactly_one_creature),
                effect: make_demon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if you control exactly one creature…"
fn if_exactly_one_creature(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let filter = ObjectFilter::creature();
    conditions::you_control_at_least(s, you, &filter, 1)
        && conditions::you_control_at_most(s, you, &filter, 1)
}

/// "…create a 5/5 black Demon creature token with flying."
fn make_demon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let demon = reg.interner().lookup("Demon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: demon,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
