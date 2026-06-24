//! Kinetic Augur — `{3}{R}` */4 Human Shaman with Trample.
//!
//! Trample.
//! Kinetic Augur's power is equal to the number of instant and sorcery cards
//! in your graveyard.
//! When this creature enters, discard up to two cards, then draw that many
//! cards.
//!
//! Trample is wired and the `*` power is set via PtValue::Star. The CDA
//! ("power equal to instant/sorcery cards in your graveyard") is wired at
//! Layer 7a via a SelfEntersBattlefield self_pt_cda (id 2). The ETB "discard
//! up to two, then draw that many" couples a variable draw count to a
//! player-chosen discard count, which is not expressible (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kinetic Augur");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA "power = instant/sorcery cards in your graveyard" — resolved at
        // Layer 7a by install_cda (id 2); toughness is the printed fixed 4.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
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
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Layer 7a self-CDA: power = instant/sorcery cards in your graveyard.
/// The CDA SETS both base power and toughness, so the compute returns the
/// printed fixed toughness (4) for the second tuple element.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            instant_sorcery_gy_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = instant and sorcery cards in your graveyard; toughness fixed 4.
fn instant_sorcery_gy_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter =
        ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let n = script::graveyard_matching(s, &filter, who, who) as i32;
    (n, 4)
}

fn etb_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard up to two cards, then draw that many cards" — the draw
    // count is the player-chosen discard count; this coupling is not
    // expressible.
    Vec::new()
}
