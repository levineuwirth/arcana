//! Consuming Blob — `{3}{G}{G}` */*+1 Ooze.
//! * "Consuming Blob's power is equal to the number of card types among
//!   cards in your graveyard and its toughness is equal to that number
//!   plus 1." — a characteristic-defining ability (*/*+1). The base
//!   P/T are transcribed as Star / StarPlus(1); the CDA is wired at
//!   Layer 7a via a `self_pt_cda` installed on ETB.
//! * "At the beginning of your end step, create a green Ooze creature
//!   token with [the same CDA]." The token is minted as a */*+1 Ooze;
//!   the token's CDA computation is GAP'd (a minted token carries no
//!   install-on-ETB trigger in this surface).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Consuming Blob");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
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
                intervening_if: None,
                effect: make_ooze_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // CDA: power = card types in your graveyard, toughness = +1.
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

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = number of card types among cards in YOUR graveyard;
// toughness = that number plus 1.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
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
    for o in s.objects.objects_in_zone(Zone::Graveyard(who)) {
        seen |= o.characteristics.types.0 & CARD_TYPE_BITS;
    }
    let n = seen.count_ones() as i32;
    (n, n + 1)
}

fn make_ooze_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ooze = reg.interner().lookup("Ooze").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    // GAP: the token's "*/*+1 = card types in your graveyard" CDA is not
    // attachable to a minted token in this surface — minted with the
    // Star / StarPlus(1) base P/T but no live computation.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: ooze,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Star),
            toughness: Some(PtValue::StarPlus(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
