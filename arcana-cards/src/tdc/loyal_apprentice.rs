//! Loyal Apprentice — `{1}{R}` 2/1 Human Artificer with Haste.
//! "Lieutenant — At the beginning of combat on your turn, if you control
//! your commander, create a 1/1 colorless Thopter artifact creature token
//! with flying. That token gains haste until end of turn."
//!
//! Haste is a base keyword. "Lieutenant" is an ability word (flavor), not a
//! KeywordAbility. The combat trigger creates the Thopter token; the token is
//! minted with Flying and Haste directly (a fresh token gaining haste until
//! end of turn is equivalent to printing Haste on it). The "if you control
//! your commander" intervening-if is GAP'd — there is no commander-control
//! `conditions::` predicate, so the gate is omitted (the trigger fires
//! unconditionally, a documented fidelity partial).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loyal Apprentice");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let _thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    // GAP intervening-if: "if you control your commander" — no commander-
    // control predicate in `conditions::`; gate omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_thopter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_thopter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg
        .interner()
        .lookup("Thopter")
        .expect("Thopter interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);
    let token = TokenDefinition {
        name: thopter,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
