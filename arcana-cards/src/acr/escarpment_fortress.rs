//! Escarpment Fortress — `{4}{W}` 3/5 Wall with Defender and Reach.
//!
//! "Other creatures you control get +1/+0." — WIRED as an ETB-installed
//! `ContinuousEffect::filtered_pump` (filter: creatures you control), lasting
//! while Escarpment Fortress is on the battlefield (glorious_anthem precedent).
//! The filter matches all your creatures; the "OTHER" self-exclusion (this Wall
//! itself also matching) is the documented minor fidelity gap for filtered
//! anthems.
//! "Whenever you attack with two or more creatures, draw a card." — there is no
//! TriggerCondition variant counting the number of declared attackers (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Escarpment Fortress");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: "Whenever you attack with two or more creatures, draw a card" — no
    // TriggerCondition variant counts the number of declared attackers.
    reg.register(
        CardDefinition::new(name, chars)
            // "Other creatures you control get +1/+0." installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_other_creatures_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "creatures you control get +1/+0" anchored to Escarpment Fortress,
/// lasting while it remains on the battlefield.
fn install_other_creatures_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
