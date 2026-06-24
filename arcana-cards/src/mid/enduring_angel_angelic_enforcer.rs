//! Enduring Angel // Angelic Enforcer — `{2}{W}{W}{W}` Creature — Angel 3/3 (transform TDFC).
//!
//! Front face (Enduring Angel):
//!   Flying, double strike.
//!   You have hexproof.
//!   If your life total would be reduced to 0 or less, instead transform this creature and your
//!   life total becomes 3. Then if this creature didn't transform this way, you lose the game.
//!
//! Back face (Angelic Enforcer):
//!   Flying.
//!   You have hexproof.
//!   Angelic Enforcer's power and toughness are each equal to your life total.
//!   Whenever this creature attacks, double your life total.
//!
//! GAP: "You have hexproof" (player-granted hexproof) — no static-ability mechanism to grant a
//!      keyword to a player.
//! GAP: front-face life-total replacement ("if your life total would be reduced to 0 or less,
//!      instead transform … and your life total becomes 3; else you lose the game") — no
//!      replacement effect for life-total reduction; not expressible.
//!
//! Back-face "power and toughness are each equal to your life total" is a Layer 7a self-CDA
//! (`ContinuousEffect::self_pt_cda` reading your life total — a state scalar), installed on ETB
//! and gated to the back face via `Duration::WhileSourceShowsFace(1)`; the back bones carry
//! PtValue::Star.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enduring Angel");
    let angel_sub = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // Back face: Angelic Enforcer — Angel with Flying.
    let back_name = reg.interner_mut().intern("Angelic Enforcer");
    let back_angel_sub = reg.interner_mut().intern("Angel");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_angel_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // P/T each equal to your life total (Layer 7a self-CDA, installed on
            // ETB, gated to the back face); Star marks the bones.
            power: Some(PtValue::Star),
            toughness: Some(PtValue::Star),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back-face only: "Whenever this creature attacks, double your life total."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_double_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1)
            // Back-face CDA: P/T each equal to your life total (live only while
            // showing the back face). Installed on ETB; the Duration gates it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_life_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: "You have hexproof" on both faces — player-granted keyword not expressible.
        // GAP: front-face life-reduction replacement → transform / lose the game not expressible.
    )
}

/// Layer 7a self-CDA gated to the back face (Angelic Enforcer): P/T each equal
/// to your life total.
fn install_life_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            life_total_pt,
            Duration::WhileSourceShowsFace(1),
        ),
    }]
}

/// P/T = your (the source controller's) life total, clamped at 0.
fn life_total_pt(state: &GameState, source: ObjectId) -> (i32, i32) {
    let who = state.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = script::life(state, who).max(0);
    (n, n)
}

fn attack_double_life(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let life = script::life(state, trig.controller).max(0) as u32;
    vec![Effect::SetLifeTotal {
        player: trig.controller,
        amount: life.saturating_mul(2),
    }]
}
