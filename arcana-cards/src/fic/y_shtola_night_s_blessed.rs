//! Y'shtola, Night's Blessed — `{1}{W}{U}{B}` 2/4 Legendary Cat Warlock.
//! Vigilance.
//! At the beginning of each end step, if a player lost 4 or more life this turn,
//! you draw a card.
//! Whenever you cast a noncreature spell with mana value 3 or greater, Y'shtola
//! deals 2 damage to each opponent and you gain 2 life.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Y'shtola, Night's Blessed");
    let cat = reg.interner_mut().intern("Cat");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of each end step, IF a player lost 4+ life this
            // turn, you draw a card.
            // GAP: no conditions helper for "a player lost 4+ life this turn";
            //      intervening-if left None (fires unconditionally each end step).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: end_step_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Whenever you cast a noncreature spell with mv 3+, deal 2 to each
            // opponent and gain 2 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .without_types(TypeLine::CREATURE.into())
                            .with_min_cmc(3),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_payoff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

fn cast_payoff(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 2,
        })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    vec![Effect::Sequence(effects)]
}
