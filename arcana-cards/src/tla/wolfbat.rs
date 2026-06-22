//! Wolfbat — `{2}{B}` 2/2 Creature — Wolf Bat.
//! Flying.
//! Whenever you draw your second card each turn, you may pay {B}. If you
//! do, return this card from your graveyard to the battlefield with a
//! finality counter on it.
//!
//! The "second card each turn" gate is enforced via an intervening-if on
//! the per-draw trigger: it fires only when exactly two cards have been
//! drawn by you this turn. The trigger lives in the graveyard zone
//! (encoding "from your graveyard"). The "you may pay {B}" gate uses
//! `OptionalPayment`; on pay, the card returns to the battlefield with a
//! finality counter (a named counter — `CounterKind::Named("finality")`).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolfbat");
    let wolf = reg.interner_mut().intern("Wolf");
    let bat = reg.interner_mut().intern("Bat");
    // Pre-intern the finality-counter name so the resolver's lookup succeeds.
    let _finality = reg.interner_mut().intern("finality");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    subtypes.0.insert(bat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::You,
            },
            intervening_if: Some(if_second_draw),
            effect: pay_b_reanimate_self,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_second_draw(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    script::cards_drawn_this_turn(s, you) == 2
}

fn pay_b_reanimate_self(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let finality = reg.interner().lookup("finality");
    let mut after = vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }];
    if let Some(sym) = finality {
        after.push(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(sym),
            count: 1,
        });
    }
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{B}").expect("valid cost")),
        then: Box::new(Effect::Sequence(after)),
        else_effect: None,
    }]
}
