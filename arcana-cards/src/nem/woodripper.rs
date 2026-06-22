//! Woodripper — `{3}{G}{G}` 4/6 Beast.
//!
//! * Fading 3 (enters with three fade counters; remove one each upkeep
//!   or sacrifice it). Modeled as the parametrized keyword.
//! * `{1}, Remove a fade counter from this creature: Destroy target
//!   artifact.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Woodripper");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Fading(3)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, Remove a fade counter from this creature: Destroy target artifact.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                remove_self_counter: Some((CounterKind::Fade, 1)),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: destroy_artifact,
        }),
    )
}

fn destroy_artifact(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
