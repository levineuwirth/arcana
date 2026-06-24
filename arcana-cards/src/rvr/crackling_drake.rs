//! Crackling Drake — `{U}{U}{R}{R}` */4 Drake with Flying.
//!
//! * Flying.
//! * Crackling Drake's power is equal to the total number of instant
//!   and sorcery cards you own in exile and in your graveyard.
//!   Wired at Layer 7a via `ContinuousEffect::self_pt_cda` on a
//!   `SelfEntersBattlefield` trigger; bones kept as */4 via PtValue::Star.
//!   Only power is `*`, so the compute returns the printed fixed
//!   toughness (4) as the second tuple element.
//! * When this creature enters, draw a card.

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
    let name = reg.interner_mut().intern("Crackling Drake");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{R}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw,
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

fn etb_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

/// "Crackling Drake's power is equal to the total number of instant and
/// sorcery cards you own in exile and in your graveyard" — install the
/// self-CDA at Layer 7a.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            inst_sorc_exile_gy_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = instant/sorcery cards you own in exile + in your graveyard;
/// toughness fixed 4. The graveyard half reuses `graveyard_matching`; the
/// exile zone is a single shared zone (not owner-partitioned), so its
/// instant/sorcery cards are counted manually, filtered by owner.
fn inst_sorc_exile_gy_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter =
        ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let gy = script::graveyard_matching(s, &filter, who, who) as i32;
    let exile = s
        .objects
        .objects_in_zone(Zone::Exile)
        .filter(|o| {
            o.owner == who
                && (o.characteristics.types.is_instant() || o.characteristics.types.is_sorcery())
        })
        .count() as i32;
    (gy + exile, 4)
}
