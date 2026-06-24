//! Filigree Attendant — `{2}{U}{U}` */3 Artifact Creature — Homunculus.
//!
//! Oracle:
//! * Flying.
//! * "Filigree Attendant's power is equal to the number of artifacts you
//!   control." — a characteristic-defining ability; power is marked `*`
//!   (PtValue::Star) and wired via a self-CDA installed on ETB
//!   (ContinuousEffect::self_pt_from_match at Layer 7a). The printed `3`
//!   toughness is preserved (the SET writes both P/T, so the compute returns
//!   the fixed toughness).

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
    let name = reg.interner_mut().intern("Filigree Attendant");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
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

/// Layer 7a self-CDA. Only POWER is `*` (= the number of artifacts you
/// control); toughness is the printed fixed `3`. The 7a SET writes BOTH P/T,
/// so a scalar compute is used that returns the artifact count for power and
/// the printed fixed toughness — `self_pt_from_match` would (wrongly) set
/// toughness to the count too.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = number of artifacts you control; toughness = printed 3.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(s, &filter, who) as i32;
    (n, 3)
}
