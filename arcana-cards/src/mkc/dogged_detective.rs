//! Dogged Detective — `{1}{B}` 2/1 Human Rogue.
//! "When this creature enters, surveil 2."
//! "Whenever an opponent draws their second card each turn, you may
//! return this card from your graveyard to your hand."
//!
//! Surveil is not an evergreen KeywordAbility variant — the printed
//! "Surveil" keyword line is realised as the ETB Effect::Surveil below
//! (keywords vec stays empty). The ETB surveil-2 trigger is wired
//! faithfully. The graveyard-recursion trigger fires on an opponent
//! drawing a card (from the graveyard zone) and returns this card to
//! hand. NOTE: the "their SECOND card each turn" restriction cannot be
//! pinned to the specific drawing opponent (the intervening-if only
//! exposes this card's controller, not the triggering player), so the
//! closest available form — any opponent draw — is used.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dogged Detective");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_surveil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: "their second card each turn" precision — cannot pin the
                // draw-count to the triggering opponent; fires on any opponent draw.
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: recur_self,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_surveil(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: trig.controller,
        count: 2,
    }]
}

fn recur_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand {
        target: trig.source,
    }]
}
