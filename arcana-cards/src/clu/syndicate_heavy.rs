//! Syndicate Heavy — `{2}{W/B}{W/B}` 4/4 black/white Giant Rogue.
//!
//! * Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each
//!   opponent loses 1 life and you gain that much life.) — modeled as a
//!   SpellCast trigger with an OptionalPayment gate. Fidelity note: the gain
//!   is fixed at the number of opponents (1 life each), which matches a
//!   two-player game; for the "you gain that much life" total we sum 1 per
//!   opponent.
//! * At the beginning of each end step, if you gained 4 or more life this
//!   turn, investigate. — Investigate is a Clue commodity token. The
//!   intervening-if ("you gained 4 or more life this turn") has no available
//!   conditions/script predicate, so it is GAP'd (fires unconditionally).
//!
//! The "Investigate" / "Extort" keyword line has no KeywordAbility variants
//! (`keywords: vec![]`); both are realized as abilities below.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{CommodityToken, Effect};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syndicate Heavy");
    let giant = reg.interner_mut().intern("Giant");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W/B}{W/B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: extort,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if "if you gained 4 or more life this turn"
                // has no conditions/script predicate for life gained this turn.
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Extort: you may pay {W/B}; if you do, each opponent loses 1 life and you
/// gain that much life.
fn extort(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let total = opponents.len() as u32;
    if total == 0 {
        return Vec::new();
    }
    let mut steps: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 1 })
        .collect();
    steps.push(Effect::GainLife { player: trig.controller, amount: total });
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{W/B}").expect("valid cost")),
        then: Box::new(Effect::Sequence(steps)),
        else_effect: None,
    }]
}

/// Investigate — create a Clue token.
fn investigate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
