//! Cosmic Spider-Man — `{W}{U}{B}{R}{G}` 5/5 Legendary Spider Human Hero.
//! Flying, first strike, trample, lifelink, haste.
//! At the beginning of combat on your turn, other Spiders you control gain
//! flying, first strike, trample, lifelink, and haste until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cosmic Spider-Man");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Trample,
            KeywordAbility::Lifelink,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: buff_other_spiders,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn buff_other_spiders(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter =
        script::subtype_filter(reg, "Spider").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let effects: Vec<Effect> = ids
        .into_iter()
        .filter(|id| *id != trig.source)
        .map(|id| Effect::Pump {
            target: id,
            power: 0,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![
                KeywordAbility::Flying,
                KeywordAbility::FirstStrike,
                KeywordAbility::Trample,
                KeywordAbility::Lifelink,
                KeywordAbility::Haste,
            ],
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
