//! Cephalopod Sentry — `{2}{W}{U}` */5 Artifact Creature — Phyrexian Squid.
//!
//! Oracle:
//! * Flying
//! * Cephalopod Sentry's power is equal to the number of artifacts you control.
//!
//! The variable power is a characteristic-defining ability (CR 604.3),
//! wired at Layer 7a via `ContinuousEffect::self_pt_cda` on a
//! `SelfEntersBattlefield` trigger. `PtValue::Star` is kept as the
//! faithful `*` power; only power is `*`, so the compute returns the
//! printed fixed toughness (5) for the second tuple element (the SET
//! overwrites both base P and T).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cephalopod Sentry");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let squid = reg.interner_mut().intern("Squid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(squid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
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

/// "Cephalopod Sentry's power is equal to the number of artifacts you
/// control" — install the self-CDA at Layer 7a.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            artifacts_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = artifacts you control; toughness fixed 5.
fn artifacts_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(s, &filter, who) as i32;
    (n, 5)
}
