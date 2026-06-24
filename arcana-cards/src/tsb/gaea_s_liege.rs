//! Gaea's Liege — `{3}{G}{G}{G}` */* Avatar. P/T equal to Forests you control
//! (or, while attacking, Forests the defending player controls); `{T}: Target
//! land becomes a Forest until this creature leaves the battlefield.`
//!
//! The self-CDA is wired at Layer 7a via `ContinuousEffect::self_pt_from_match`
//! counting Forests you control (the default, non-attacking value), installed
//! on an ETB trigger. The "while attacking, count the DEFENDING player's
//! Forests instead" clause swaps which player the count is relative to based
//! on combat context, which a static count_filter can't express — that
//! context-switch refinement stays unmodeled.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gaea's Liege");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // Self-CDA — P/T equal to the number of Forests you control (the
        // default value), installed at Layer 7a on ETB below. The "while
        // attacking, count the defending player's Forests" refinement is a
        // combat-context swap a static count_filter can't express.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Target land becomes a Forest until this creature leaves the battlefield.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: become_forest,
        }),
    )
}

/// "Gaea's Liege's power and toughness are each equal to the number of
/// Forests you control" — install the self-CDA at Layer 7a (the default,
/// non-attacking value).
fn install_cda(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Forest")
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn become_forest(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target land becomes a Forest" — no subtype-add effect primitive
    // (AddType adds a TypeLine type, not a land subtype), and the
    // until-this-leaves duration likewise has no matching primitive.
    Vec::new()
}
