//! Spellheart Chimera — `{1}{U}{R}` */3 Chimera with Flying and Trample.
//! "Spellheart Chimera's power is equal to the number of instant and sorcery
//!  cards in your graveyard."
//!
//! Power is a characteristic-defining ability, emitted as `PtValue::Star`;
//! the count-instants/sorceries-in-graveyard CDA is wired at Layer 7a via
//! `ContinuousEffect::self_pt_cda` on a `SelfEntersBattlefield` trigger.

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
    let name = reg.interner_mut().intern("Spellheart Chimera");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
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

/// "Spellheart Chimera's power is equal to the number of instant and sorcery
/// cards in your graveyard" — install the self-CDA at Layer 7a. The CDA SETS
/// both base power and toughness, so the compute returns the printed fixed
/// toughness (3) for the second tuple element.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            instant_sorcery_gy_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = instant and sorcery cards in your graveyard; toughness fixed 3.
fn instant_sorcery_gy_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter =
        ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let n = script::graveyard_matching(s, &filter, who, who) as i32;
    (n, 3)
}
