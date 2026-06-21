//! Safana, Calimport Cutthroat — `{2}{B}` 3/2 Legendary Human Rogue
//! with Menace.
//!
//! * Menace — keyword line. ("Choose a Background" is a deck-building
//!   keyword, not a usable `KeywordAbility` variant — GAP'd; "Treasure"
//!   in the Scryfall keyword list is the produced token, not a keyword.)
//! * "At the beginning of your end step, if you have the initiative,
//!   create a Treasure token. Create three of those tokens instead if
//!   you've completed a dungeon." → a `StepBegins(End, You)` trigger
//!   creating a Treasure (`Effect::CreateCommodityToken`). GAP: there is
//!   no "you have the initiative" condition helper (so the trigger
//!   over-fires — it can't be gated on initiative) and no
//!   "completed a dungeon" predicate (so always one Treasure, never the
//!   three-token escalation).
//! * GAP: "Choose a Background" — deck-building keyword, not expressible.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Safana, Calimport Cutthroat");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
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
                // GAP: "if you have the initiative" intervening-if — no
                // initiative condition helper exists.
                intervening_if: None,
                effect: end_step_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Create three of those tokens instead if you've completed a
    // dungeon." — no completed-a-dungeon predicate; always one Treasure.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
