//! Ouphe Vandals — `{2}{G}` 2/2 green Ouphe Rogue.
//! "{G}, Sacrifice this creature: Counter target activated ability from an artifact source
//! and destroy that artifact if it's on the battlefield."
//! Wired via TargetFilter::AbilityOnStack (artifact source_filter) + Effect::Counter,
//! which handles ability stack entries; the source artifact is looked up from the
//! targeted stack entry and destroyed if it's still on the battlefield.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Ouphe Vandals");
    let ouphe = reg.interner_mut().intern("Ouphe");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ouphe);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}, Sacrifice this creature: Counter target activated ability from an artifact source and destroy that artifact if it's on the battlefield.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AbilityOnStack {
                        activated: true,
                        triggered: false,
                        source_filter: Some(
                            ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                        ),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_artifact_ability,
            }),
    )
}

fn counter_artifact_ability(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::Counter { target: *id }];
    // "…and destroy that artifact if it's on the battlefield": read the
    // targeted ability entry's source off the stack at resolution.
    if let Some(entry) = state.stack.iter().find(|e| e.id == *id) {
        if state
            .objects
            .get(entry.source)
            .is_some_and(|o| o.is_permanent_on_battlefield())
        {
            effects.push(Effect::DestroyPermanent { target: entry.source });
        }
    }
    effects
}
