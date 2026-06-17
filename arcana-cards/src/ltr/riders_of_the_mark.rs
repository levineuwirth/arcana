//! Riders of the Mark — `{6}{R}` 7/4 Human Knight with Trample and Haste.
//! Affinity for Humans is a cost-reduction keyword with no expressible variant — GAP.
//! "At the beginning of your end step, if this creature attacked this turn, return
//! it to its owner's hand. If you do, create a number of 1/1 white Human Soldier
//! creature tokens equal to its toughness."
//! The "if it attacked this turn" intervening-if has no listed conditions helper,
//! so the gate is a GAP (left None); the bounce + token body is wired, with the
//! token count computed from its toughness at resolution.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riders of the Mark");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Affinity for Humans — cost-reduction keyword not expressible.
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "if this creature attacked this turn" — no
                // listed conditions helper for source-attacked-this-turn.
                intervening_if: None,
                effect: bounce_and_make_soldiers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn bounce_and_make_soldiers(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::toughness_of(state, trig.source).max(0) as u32;
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut effects = vec![Effect::ReturnToHand { target: trig.source }];
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        if let Some(human) = reg.interner().lookup("Human") {
            subtypes.0.insert(human);
        }
        subtypes.0.insert(soldier);
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: soldier,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
