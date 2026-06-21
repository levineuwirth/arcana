//! Persistent Marshstalker — `{1}{B}` 3/1 black Rat Berserker.
//!
//! Oracle:
//! * "This creature gets +1/+0 for each other Rat you control." — a pure
//!   dynamic continuous static; no expressible self-pump-per-other-Rat
//!   primitive. GAP'd.
//! * Threshold keyword has no `KeywordAbility` variant — GAP'd (the
//!   threshold gate is expressed as the trigger's intervening-if below).
//! * "Threshold — Whenever you attack with one or more Rats, if there are
//!   seven or more cards in your graveyard, you may pay {2}{B}. If you do,
//!   return this card from your graveyard to the battlefield tapped and
//!   attacking." — a graveyard-zoned attack trigger gated by an
//!   intervening-if (graveyard >= 7), whose effect offers an optional
//!   {2}{B} payment to return this card from the graveyard. FIDELITY GAP:
//!   `ReturnFromGraveyardToBattlefield` returns it to the battlefield but
//!   not positioned "tapped and attacking" (no graveyard->battlefield
//!   tapped-attacking primitive); the return itself is faithful.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Persistent Marshstalker");
    let rat = reg.interner_mut().intern("Rat");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: keyword — Threshold has no KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "+1/+0 for each other Rat you control" (no dynamic
    // self-pump-per-other-subtype primitive).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: Some(if_seven_in_graveyard),
                effect: maybe_return_self,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_seven_in_graveyard(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::graveyard_size(s, you) >= 7
}

fn maybe_return_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{B}").expect("valid cost")),
        then: Box::new(Effect::ReturnFromGraveyardToBattlefield {
            target: trig.source,
        }),
        else_effect: None,
    }]
}
