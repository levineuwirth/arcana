//! Primalcrux — `{G}{G}{G}{G}{G}{G}` */* Elemental with Trample.
//! "Chroma — This creature's power and toughness are each equal to the
//!  number of green mana symbols in the mana costs of permanents you
//!  control."
//!
//! Chroma-green is exactly devotion to green (CR 700.5: green mana symbols
//! among the mana costs of permanents you control), installed as a Layer 7a
//! self-CDA on ETB via `ContinuousEffect::self_pt_cda` over
//! `script::devotion`. PtValue::Star marks the `*/*` bones.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Primalcrux");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{G}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
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

/// Layer 7a self-CDA: P/T each equal to your devotion to green (Chroma —
/// green).
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            chroma_green,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = the number of green mana symbols among the mana costs of permanents
/// you control (devotion to green).
fn chroma_green(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = script::devotion(s, who, ColorSet::green()) as i32;
    (n, n)
}
