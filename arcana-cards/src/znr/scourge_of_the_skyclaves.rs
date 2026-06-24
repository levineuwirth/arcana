//! Scourge of the Skyclaves — `{1}{B}` */* black Demon.
//!
//! Oracle:
//! * Scourge of the Skyclaves's power and toughness are each equal to 20 minus
//!   the highest life total among players — a characteristic-defining ability
//!   wired at Layer 7a via a `SelfEntersBattlefield` `self_pt_cda`. Both `*`
//!   axes resolve to `20 - max(life)` (clamped at 0; CR 107.1b — P/T are never
//!   negative for this purpose, and the SET cannot reduce below the scalar).
//! * GAP (keyword): Kicker {4}{B} — Kicker is not in the usable KeywordAbility
//!   surface.
//! * GAP (trigger): "When you cast this spell, if it was kicked, each player
//!   loses half their life, rounded up." — the "was kicked" precondition is not
//!   trackable (no Kicker support); the cast-trigger is GAP'd whole rather than
//!   firing unconditionally.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scourge of the Skyclaves");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // `*/*` — both axes are a CDA scalar (20 minus the highest life total
        // among players). Resolved at Layer 7a by install_cda.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![],
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

/// Layer 7a self-CDA: P/T each equal to 20 minus the highest life total.
fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = 20 minus the highest life total among players (clamped at 0).
fn cda_pt(s: &GameState, _source: ObjectId) -> (i32, i32) {
    let highest = (0..s.num_players())
        .map(|p| s.player(p).life)
        .max()
        .unwrap_or(0);
    let n = (20 - highest).max(0);
    (n, n)
}
