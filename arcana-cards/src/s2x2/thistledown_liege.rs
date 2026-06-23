//! Thistledown Liege — `{1}{W/U}{W/U}{W/U}` 1/3 Kithkin Knight with Flash.
//! "Other white creatures you control get +1/+1."
//! "Other blue creatures you control get +1/+1."
//!
//! The two static color anthems are wired as a single
//! `SelfEntersBattlefield` trigger that installs two
//! `ContinuousEffect::filtered_pump` effects — one over white creatures
//! you control, one over blue — each lasting while this creature is on the
//! battlefield. (`with_colors` requires the named color bit present, so a
//! W/U creature matches both, exactly per the oracle. The "other"
//! exclusion of this creature itself is a documented minor fidelity gap;
//! this Liege is W/U, so it sees +2/+2 from its own anthems — slightly
//! generous but the closest expressible reading.)

use arcana_core::effects::KeywordAbility;
use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Thistledown Liege");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W/U}{W/U}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_color_anthems,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install both "white creatures you control get +1/+1" and "blue
/// creatures you control get +1/+1" anchored to this creature, lasting
/// until it leaves the battlefield.
fn install_color_anthems(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let white_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_colors(ColorSet::white());
    let blue_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_colors(ColorSet::blue());
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                white_filter,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                blue_filter,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
