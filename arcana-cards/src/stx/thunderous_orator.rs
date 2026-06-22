//! Thunderous Orator — `{1}{W}` 2/2 Kor Wizard with Vigilance.
//! "Whenever this creature attacks, it gains flying until end of turn if
//! you control a creature with flying. The same is true for first strike,
//! double strike, deathtouch, indestructible, lifelink, menace, and
//! trample."
//!
//! Vigilance is wired. The attack trigger grants this creature each of
//! the eight keywords until end of turn, individually gated on
//! controlling a creature that already has that keyword.

use arcana_core::effects::{Condition, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thunderous Orator");
    let kor = reg.interner_mut().intern("Kor");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: conditional_keyword_grants,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn conditional_keyword_grants(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    const KEYWORDS: [KeywordAbility; 8] = [
        KeywordAbility::Flying,
        KeywordAbility::FirstStrike,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Deathtouch,
        KeywordAbility::Indestructible,
        KeywordAbility::Lifelink,
        KeywordAbility::Menace,
        KeywordAbility::Trample,
    ];
    KEYWORDS
        .into_iter()
        .map(|kw| Effect::Conditional {
            condition: Condition::ControlPermanentMatching(
                ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_keyword(kw.clone()),
            ),
            then: Box::new(Effect::GrantKeyword {
                target: trig.source,
                keyword: kw,
                duration: Duration::EndOfTurn,
            }),
            otherwise: None,
        })
        .collect()
}
