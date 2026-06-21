//! Tivash, Gloom Summoner — `{4}{B}` 4/4 Legendary Human Warlock with
//! Lifelink.
//!
//! "At the beginning of your end step, if you gained life this turn, you may
//! pay X life, where X is the amount of life you gained this turn. If you do,
//! create an X/X black Demon creature token with flying."
//!
//! The end-step trigger is gated by an intervening-if for "you gained life
//! this turn"; at resolution X is computed (life gained this turn) and an
//! OptionalPayment of X life conditionally mints an X/X flying Demon.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::conditions;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tivash, Gloom Summoner");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let _demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_gained_life),
            effect: maybe_make_demon,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_gained_life(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_gained_life_this_turn(s, you)
}

fn maybe_make_demon(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::life_gained_this_turn(state, trig.controller);
    if x == 0 {
        return Vec::new();
    }
    let demon = reg.interner().lookup("Demon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let token = TokenDefinition {
        name: demon,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(x as i32)),
        toughness: Some(PtValue::Fixed(x as i32)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Life(x),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token,
        }),
        else_effect: None,
    }]
}
