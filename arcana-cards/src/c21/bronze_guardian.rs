//! Bronze Guardian — `{4}{W}` */5 Artifact Creature — Golem.
//! "Double strike
//!  Ward {2}
//!  Other artifacts you control have ward {2}.
//!  Bronze Guardian's power is equal to the number of artifacts you
//!  control."
//!
//! Double strike and Ward {2} are keywords. "Other artifacts you control
//! have ward {2}" is a static keyword-granting buff with no demonstrated
//! hook on this shape — GAP. The characteristic-defining "power equal to the
//! number of artifacts you control" is installed at Layer 7a via an ETB
//! self-CDA (scalar compute); toughness is the printed fixed 5, returned
//! unchanged so the 7a SET preserves it.

// GAP (static): "Other artifacts you control have ward {2}" — no
// keyword-granting anthem hook on this shape.

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
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bronze Guardian");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::DoubleStrike,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
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

fn install_cda(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == who && o.characteristics.types.is_artifact())
        .count() as i32;
    // Only power is `*`; the SET overwrites both, so return the printed 5.
    (n, 5)
}
