//! Memnarchitect — `{1}{U}` 1/4 Vedalken Artificer.
//!
//! "Spells you cast have affinity for artifact creatures that weren't
//! originally artifact creatures.
//! {1}{U}{U}: Target permanent becomes an artifact in addition to its
//! other types."
//!
//! The affinity static is a cost-reduction rule with no primitive in
//! this API surface — GAP'd. The activated ability makes a target
//! permanent an artifact in addition to its types (no stated duration →
//! permanent).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Memnarchitect");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static — "Spells you cast have affinity for artifact creatures
    // that weren't originally artifact creatures" (no affinity / dynamic
    // cost-reduction primitive in this API surface).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}{U}: Target permanent becomes an artifact in addition to its other types.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_artifact,
            }),
    )
}

fn make_artifact(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddType {
        target: *id,
        types: TypeLine::ARTIFACT.into(),
        duration: Duration::Permanent,
    }]
}
