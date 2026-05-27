//! Sproutwatch Dryad — `{1}{G}{G}` 3/3 green Dryad.
//! "At the beginning of each combat, Sproutwatch Dryad gains flying until end
//! of turn if a creature you control or a card in your hand has flying. The
//! same is true for first strike, double strike, deathtouch, haste, hexproof,
//! indestructible, lifelink, menace, reach, trample, and vigilance."
//!
//! GAP: "if a creature you control or a card in your hand has [keyword]"
//! — checking whether any permanent/hand card has a specific keyword is
//! not available in the script helpers. Emitting unconditional keyword grants
//! as best-effort (over-triggers).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sproutwatch Dryad");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: gain_keywords,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_keywords(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: conditional on whether any creature/hand card has each keyword —
    // not checkable; granting all listed keywords unconditionally as best-effort.
    let keywords = vec![
        KeywordAbility::Flying,
        KeywordAbility::FirstStrike,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Deathtouch,
        KeywordAbility::Haste,
        KeywordAbility::Hexproof,
        KeywordAbility::Indestructible,
        KeywordAbility::Lifelink,
        KeywordAbility::Menace,
        KeywordAbility::Reach,
        KeywordAbility::Trample,
        KeywordAbility::Vigilance,
    ];
    keywords
        .into_iter()
        .map(|kw| Effect::GrantKeyword {
            target: trig.source,
            keyword: kw,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
