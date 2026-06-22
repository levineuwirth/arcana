//! Mistfire Weaver — `{3}{U}` 3/1 Djinn Wizard with Flying.
//! Morph {2}{U}.
//! "When this creature is turned face up, target creature you control
//! gains hexproof until end of turn."
//!
//! Flying is wired. Morph (its face-down cast + face-up cost) is not in
//! the usable keyword surface and has no morph-cost field — GAP'd. The
//! "turned face up" trigger has no dedicated TriggerCondition;
//! `SelfTransforms` is the closest available (Fathom Seer precedent),
//! and its hexproof grant is wired faithfully.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mistfire Weaver");
    let djinn = reg.interner_mut().intern("Djinn");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Morph {2}{U} — Morph is not in the usable keyword surface
        // (no morph-cost field).
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "when turned face up" has no dedicated
                // TriggerCondition; SelfTransforms is the closest available.
                trigger_condition: TriggerCondition::SelfTransforms { to_face: None },
                intervening_if: None,
                effect: grant_hexproof,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn grant_hexproof(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Hexproof,
        duration: Duration::EndOfTurn,
    }]
}
