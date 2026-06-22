//! Battle Angels of Tyr — `{2}{W}{W}` 4/4 Angel Knight.
//!
//! Oracle:
//! * Flying — `KeywordAbility::Flying`. Myriad has no `KeywordAbility`
//!   variant; GAP'd. (The "Treasure" Scryfall keyword is the token it makes,
//!   handled in the trigger below, not a creature keyword.)
//! * "Whenever this creature deals combat damage to a player, draw a card if
//!   that player has more cards in hand than each other player. Then you
//!   create a Treasure token if that player controls more lands than each
//!   other player. Then you gain 3 life if that player has more life than
//!   each other player." — a combat-damage-to-a-player trigger whose three
//!   riders each compare the damaged player against every other player
//!   (hand size / land count / life total).

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battle Angels of Tyr");
    let angel = reg.interner_mut().intern("Angel");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: keyword — Myriad (no KeywordAbility variant).
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_riders,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn lands_of(state: &GameState, p: PlayerId) -> u32 {
    let land_filter =
        ObjectFilter::permanent().with_types(TypeLine::LAND.into()).controlled_by(ControllerConstraint::You);
    script::count_matching(state, &land_filter, p)
}

fn combat_damage_riders(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(dealt_to) = trig.damaged_player() else {
        return Vec::new();
    };
    let others: Vec<PlayerId> = script::all_players(state)
        .into_iter()
        .filter(|&p| p != dealt_to)
        .collect();

    let mut effects = Vec::new();

    // "draw a card if that player has more cards in hand than each other player"
    let hand = script::hand_size(state, dealt_to);
    if others.iter().all(|&p| hand > script::hand_size(state, p)) {
        effects.push(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        });
    }

    // "create a Treasure token if that player controls more lands than each other player"
    let lands = lands_of(state, dealt_to);
    if others.iter().all(|&p| lands > lands_of(state, p)) {
        effects.push(Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        });
    }

    // "gain 3 life if that player has more life than each other player"
    let life = script::life(state, dealt_to);
    if others.iter().all(|&p| life > script::life(state, p)) {
        effects.push(Effect::GainLife {
            player: trig.controller,
            amount: 3,
        });
    }

    effects
}
