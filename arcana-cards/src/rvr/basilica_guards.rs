//! Basilica Guards — `{2}{W}` 1/4 Creature — Human Soldier.
//! Defender.
//! Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each opponent
//! loses 1 life and you gain that much life.)
//!   Extort isn't in the KeywordAbility surface, but its rules are fully expressible:
//!   a SpellCast trigger gated by an OptionalPayment of {W/B}; on pay, each opponent
//!   loses 1 life and you gain that much life (= number of opponents).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basilica Guards");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
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
            }),
    )
}

fn extort(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let n = opponents.len() as u32;
    let mut drain: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 1 })
        .collect();
    drain.push(Effect::GainLife { player: trig.controller, amount: n });
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{W/B}").expect("valid cost")),
        then: Box::new(Effect::Sequence(drain)),
        else_effect: None,
    }]
}
