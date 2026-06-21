//! Ethereal Investigator — `{3}{U}` 2/3 Spirit with Flying.
//! "When this creature enters, investigate X times, where X is the
//! number of opponents you have."
//! "Whenever you draw your second card each turn, create a 1/1 white
//! Spirit creature token with flying."
//!
//! Flying is a base keyword. The ETB investigates once per opponent
//! (X Clue tokens). The second-draw trigger fires on each card you
//! draw, gated by an intervening-if that requires exactly two cards
//! drawn this turn.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ethereal Investigator");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_investigate_per_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_draw),
                effect: make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_investigate_per_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::opponents(state, trig.controller).len() as u32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: n,
    }]
}

fn if_second_draw(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::cards_drawn_this_turn(s, you) == 2
}

fn make_spirit(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
