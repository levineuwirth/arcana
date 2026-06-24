//! Nighthawk Scavenger — `{1}{B}{B}` 1+*/3 black Vampire Rogue with
//! Flying, Deathtouch, Lifelink.
//!
//! Oracle:
//! * Flying, deathtouch, lifelink.
//! * Power is equal to 1 plus the number of card types among cards in your
//!   opponents' graveyards.
//!
//! The keyword line is fully wired. Power is transcribed as `StarPlus(1)`
//! (`1+*`); toughness is the fixed 3.
//!
//! The CDA "power is equal to 1 plus the number of card types among cards
//! in your opponents' graveyards" is wired at Layer 7a via
//! `ContinuousEffect::self_pt_cda` on a `SelfEntersBattlefield` trigger:
//! the compute ORs the card-type bits of every card in each opponent's
//! graveyard, counts the distinct bits `n`, and sets base power to
//! `n + 1` (toughness fixed 3 — only power is `*`).

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
    let name = reg.interner_mut().intern("Nighthawk Scavenger");
    let vampire = reg.interner_mut().intern("Vampire");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::StarPlus(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Deathtouch,
            KeywordAbility::Lifelink,
        ],
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

/// "Power is equal to 1 plus the number of card types among cards in your
/// opponents' graveyards" — install the self-CDA at Layer 7a.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            opp_gy_types_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = 1 + distinct card types among cards in your opponents'
/// graveyards; toughness fixed 3.
fn opp_gy_types_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    // The eight real card types (Kindred is not a card type for this count).
    const CARD_TYPE_BITS: u16 = TypeLine::CREATURE
        | TypeLine::INSTANT
        | TypeLine::SORCERY
        | TypeLine::ENCHANTMENT
        | TypeLine::ARTIFACT
        | TypeLine::LAND
        | TypeLine::PLANESWALKER
        | TypeLine::BATTLE;
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let mut seen: u16 = 0;
    for o in s.objects.objects_in_zone_kind(ZoneKind::Graveyard) {
        if o.owner != who {
            seen |= o.characteristics.types.0 & CARD_TYPE_BITS;
        }
    }
    let n = seen.count_ones() as i32;
    (n + 1, 3)
}
