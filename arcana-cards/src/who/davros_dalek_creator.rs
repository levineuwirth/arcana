//! Davros, Dalek Creator — `{1}{U}{B}{R}` 3/4 Legendary Artifact Creature —
//! Alien Scientist with Menace.
//! "At the beginning of your end step, create a 3/3 black Dalek artifact
//!  creature token with menace if an opponent lost 3 or more life this turn.
//!  Then each opponent who lost 3 or more life this turn faces a villainous
//!  choice — You draw a card, or that player discards a card."

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Davros, Dalek Creator");
    let alien = reg.interner_mut().intern("Alien");
    let scientist = reg.interner_mut().intern("Scientist");
    let _dalek = reg.interner_mut().intern("Dalek");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(scientist);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "At the beginning of your end step, create a 3/3 black Dalek
            // artifact creature token with menace [if an opponent lost 3+ life
            // this turn]."
            // GAP: the "if an opponent lost 3 or more life this turn" intervening-
            // if gate is not in the blessed conditions surface, so the token is
            // created every end step. The "villainous choice" follow-up clause
            // (each qualifying opponent: you draw, or they discard) is also
            // omitted — there is no blessed villainous-choice effect.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_dalek,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_dalek(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let dalek = reg.interner().lookup("Dalek").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dalek);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dalek,
            colors: ColorSet::black(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Menace],
            abilities: vec![],
        },
    }]
}
