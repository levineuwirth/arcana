//! Hostile Investigator — `{3}{B}` 4/3 black Ogre Rogue Detective.
//! When this creature enters, target opponent discards a card.
//! Whenever one or more players discard one or more cards, investigate. This
//! ability triggers only once each turn.
//!
//! Investigate is not a base KeywordAbility variant (keywords: vec![]); the
//! Clue token is minted via Effect::CreateCommodityToken. The ETB targets a
//! player who discards a card. The second trigger fires on any discard, once
//! per turn, and investigates.

use arcana_core::effects::{CommodityToken, Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hostile Investigator");
    let ogre = reg.interner_mut().intern("Ogre");
    let rogue = reg.interner_mut().intern("Rogue");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(rogue);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: target_discards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn target_discards(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}

fn investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
