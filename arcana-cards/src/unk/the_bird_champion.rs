//! The Bird Champion — `{2}{G}{G}{U}{U}` 9/9 Legendary Bird Wizard
//! with Defender and Flying.
//! "At the beginning of your upkeep, create a 1/1 green and blue Bird
//! creature token with flying for each Bird you control."
//! "As long as you control twelve or more Birds, you can't lose the
//! game and opponents can't win the game."
//!
//! Defender + Flying are base keywords. The upkeep trigger mints one
//! 1/1 G/U flying Bird token per Bird you control (dynamic count). The
//! can't-lose / opponents-can't-win static has no expressible
//! primitive — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Bird Champion");
    let bird = reg.interner_mut().intern("Bird");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "as long as you control twelve or more Birds, you can't
    // lose the game and opponents can't win the game" — no expressible
    // game-rule-altering primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_bird_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_bird_tokens(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bird_filter = script::subtype_filter(reg, "Bird")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &bird_filter, trig.controller);
    let bird = match reg.interner().lookup("Bird") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
