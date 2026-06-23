//! Hold the Gates — `{2}{W}` enchantment. "Creatures you control get
//! +0/+1 for each Gate you control and have vigilance."
//!
//! Implementation: an ETB trigger installs TWO continuous effects —
//! (1) a GLOBAL PER-MATCH filtered pump giving creatures you control
//! +0/+1 for each Gate (a land subtype) you control, and (2) a
//! keyword anthem granting vigilance to creatures you control. Both
//! last while this enchantment is on the battlefield. The per-match
//! count is recursion-proof by construction (matched against BASE
//! characteristics).

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hold the Gates");
    // Intern "Gate" at registration so the resolver can `lookup` it from
    // the (immutable) registry interner without needing `&mut reg`.
    let _gate = reg.interner_mut().intern("Gate");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures you control get +0/+1 for each Gate
/// you control" (per-match) AND "creatures you control have vigilance"
/// (keyword anthem).
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let gate = reg
        .interner()
        .lookup("Gate")
        .expect("Gate subtype interned at registration");
    let your_creatures =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let your_gates = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(gate);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump_per_match(
                trig.source,
                your_creatures,
                your_gates,
                0,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::keyword_anthem(
                trig.source,
                trig.controller,
                KeywordAbility::Vigilance,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
