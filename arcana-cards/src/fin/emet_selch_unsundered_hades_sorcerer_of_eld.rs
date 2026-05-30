//! Emet-Selch, Unsundered // Hades, Sorcerer of Eld
//!
//! Front face (Emet-Selch, Unsundered — {1}{U}{B} Legendary Creature — Elder Wizard, 2/4):
//!   Vigilance
//!   Whenever Emet-Selch enters or attacks, draw a card, then discard a card.
//!   At the beginning of your upkeep, if there are fourteen or more cards in your graveyard,
//!   you may transform Emet-Selch.
//!
//! Back face (Hades, Sorcerer of Eld — Legendary Creature — Avatar):
//!   Vigilance
//!   Echo of the Lost — During your turn, you may play cards from your graveyard.
//!   If a card or token would be put into your graveyard from anywhere, exile it instead.
//!
//! GAP: "Echo of the Lost" — play cards from graveyard during your turn is not an expressible Effect.
//! GAP: Replacement effect "if a card or token would be put into your graveyard, exile it instead"
//!      is not expressible with current Effect API.
//! GAP: Keyword "Echo of the Lost" is not in KeywordAbility enum.
//! GAP: Back-face-only triggered/static abilities not auto-installed on transform.
//! GAP: "you may transform" — modeled as unconditional transform when condition is met; no optional
//!      choice modal.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
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
    let name = reg.interner_mut().intern("Emet-Selch, Unsundered");
    let elder_sub = reg.interner_mut().intern("Elder");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hades, Sorcerer of Eld");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(avatar_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Vigilance],
            // GAP: "Echo of the Lost" keyword not in KeywordAbility enum.
            // GAP: "play cards from your graveyard during your turn" not modeled.
            // GAP: "if a card/token would go to your graveyard, exile it instead" replacement not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Whenever Emet-Selch enters the battlefield, draw a card, then discard a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: draw_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Whenever Emet-Selch attacks, draw a card, then discard a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: draw_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // At the beginning of your upkeep, if 14+ cards in graveyard, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn draw_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn upkeep_transform(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Intervening-if: 14 or more cards in graveyard
    let gy_count = script::graveyard_size(state, trig.controller);
    if gy_count >= 14 {
        // GAP: "you may transform" — modeled as unconditional transform when condition met
        vec![Effect::Transform { target: trig.source }]
    } else {
        Vec::new()
    }
}
