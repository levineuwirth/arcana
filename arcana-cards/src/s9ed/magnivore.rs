//! Magnivore — `{2}{R}{R}` */* Lhurgoyf with Haste.
//!
//! Oracle:
//! * Haste.
//! * Magnivore's power and toughness are each equal to the number of sorcery
//!   cards in all graveyards. (Installed at Layer 7a via an ETB self-CDA —
//!   `self_pt_cda` counting sorcery cards across every player's graveyard;
//!   symmetric `*`/`*`. A sorcery is detected by its base type line, so no
//!   registry is needed.)

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
    let name = reg.interner_mut().intern("Magnivore");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Haste],
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

fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            sorcery_cards_in_all_graveyards,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn sorcery_cards_in_all_graveyards(s: &GameState, _source: ObjectId) -> (i32, i32) {
    let n = s
        .objects
        .objects_in_zone_kind(ZoneKind::Graveyard)
        .filter(|o| o.is_sorcery())
        .count() as i32;
    (n, n)
}
