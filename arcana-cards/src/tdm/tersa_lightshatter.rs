//! Tersa Lightshatter — `{2}{R}` 3/3 Legendary Orc Wizard with Haste.
//!
//! Oracle:
//! * Haste (keyword).
//! * When Tersa Lightshatter enters, discard up to two cards, then draw that
//!   many cards. — GAP: "discard up to N, then draw that many" couples a
//!   variable discard count to the draw count, which `Effect::Discard` /
//!   `DrawCards` (fixed counts) cannot express.
//! * Whenever Tersa Lightshatter attacks, if there are seven or more cards in
//!   your graveyard, exile a card at random from your graveyard. You may play
//!   that card this turn. — intervening-if (7+ graveyard) IS wired; the effect
//!   is GAP'd (no "exile a random card from your graveyard, you may play it"
//!   primitive — ImpulseExile works from the top of the library, not the yard).

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tersa Lightshatter");
    let orc = reg.interner_mut().intern("Orc");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: Some(if_graveyard_seven),
                effect: attack_dig_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_graveyard_seven(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::graveyard_at_least(s, you, 7)
}

fn etb_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard up to two cards, then draw that many" — variable discard
    // count coupled to the draw count is not expressible.
    Vec::new()
}

fn attack_dig_graveyard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile a card at random from your graveyard, you may play it this
    // turn" — no random-graveyard impulse primitive.
    Vec::new()
}
