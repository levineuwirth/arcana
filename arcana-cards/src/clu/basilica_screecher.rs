//! Basilica Screecher — `{1}{B}` 1/2 Bat with Flying.
//!
//! Oracle:
//! * Flying
//! * Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each
//!   opponent loses 1 life and you gain that much life.)
//!
//! "Extort" is not in the usable KeywordAbility surface, but its body IS a
//! triggered ability: on each spell you cast, you may pay {W/B}; if you
//! do, each opponent loses 1 life and you gain that much life. Modeled via
//! `SpellCast { caster: You }` → `OptionalPayment` of {W/B}, then a
//! per-opponent drain. The life gained equals the number of opponents who
//! lost life (1 each).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basilica Screecher");
    let bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
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
        }),
    )
}

/// "you may pay {W/B}. If you do, each opponent loses 1 life and you gain
/// that much life."
fn extort(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let gained = opponents.len() as u32;
    let mut drain: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 1 })
        .collect();
    drain.push(Effect::GainLife {
        player: trig.controller,
        amount: gained,
    });
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{W/B}").expect("valid cost")),
        then: Box::new(Effect::Sequence(drain)),
        else_effect: None,
    }]
}
