//! Iron-Heart Chimera — `{4}` 2/2 Artifact Creature — Chimera.
//! Vigilance.
//! Sacrifice this creature: Put a +2/+2 counter on target Chimera
//! creature. It gains vigilance. (This effect lasts indefinitely.)
//!
//! Vigilance is a base keyword. The sacrifice ability targets a Chimera
//! creature, adds a +2/+2 counter (a named counter — there is no
//! dedicated `CounterKind` for +2/+2; the demonstrated AddCounters takes
//! a CounterKind, so we use `Named("+2/+2")`), and grants vigilance with
//! `Duration::Permanent` ("lasts indefinitely").

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iron-Heart Chimera");
    let chimera = reg.interner_mut().intern("Chimera");
    let _plus = reg.interner_mut().intern("+2/+2");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let chimera_filter = ObjectFilter::new()
        .with_types(TypeLine::CREATURE.into())
        .with_subtypes_any(vec![chimera]);

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature. It gains vigilance.".into(),
            cost: ActivationCost {
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    chimera_filter.controlled_by(ControllerConstraint::Any),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: buff_chimera,
        }),
    )
}

fn buff_chimera(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let plus = reg.interner().lookup("+2/+2");
    let mut effects = Vec::new();
    if let Some(plus) = plus {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::Named(plus),
            count: 1,
        });
    }
    effects.push(Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Vigilance,
        duration: Duration::Permanent,
    });
    effects
}
