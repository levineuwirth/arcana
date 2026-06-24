//! Polygoyf — `{2}{G}` */1+* Lhurgoyf with Trample.
//!
//! Oracle:
//! * Trample — keyword. (Scryfall also lists `Myriad`; it is not a
//!   supported `KeywordAbility` and its attack-trigger token-copy fan-out
//!   is not expressible, so it is GAP'd.)
//! * Polygoyf's power is equal to the number of card types among cards in
//!   all graveyards and its toughness is equal to that number plus 1. — a
//!   characteristic-defining ability, wired at Layer 7a via a
//!   SelfEntersBattlefield self_pt_cda: the compute ORs the TypeLine bits of
//!   every card in every graveyard and counts the distinct card-type bits
//!   `n`, setting base P/T to `(n, n + 1)`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::{Zone, ZoneKind};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Polygoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA — power = card types among all graveyards, toughness = +1 —
        // resolved at Layer 7a by install_cda.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = number of card types among cards in all graveyards;
// toughness = that number plus 1.
fn cda_pt(s: &GameState, _source: ObjectId) -> (i32, i32) {
    // The eight real card types (Kindred is not a card type for this count).
    const CARD_TYPE_BITS: u16 = TypeLine::CREATURE
        | TypeLine::INSTANT
        | TypeLine::SORCERY
        | TypeLine::ENCHANTMENT
        | TypeLine::ARTIFACT
        | TypeLine::LAND
        | TypeLine::PLANESWALKER
        | TypeLine::BATTLE;
    let mut seen: u16 = 0;
    for o in s.objects.objects_in_zone_kind(ZoneKind::Graveyard) {
        seen |= o.characteristics.types.0 & CARD_TYPE_BITS;
    }
    let n = seen.count_ones() as i32;
    (n, n + 1)
}
