//! Ashnod's Transmogrant — `{1}` artifact.
//! "{T}, Sacrifice this artifact: Put a +1/+1 counter on target nonartifact
//! creature. That creature becomes an artifact in addition to its other
//! types." The counter is wired; the permanent type change is a GAP (only
//! EndOfTurn / WhileSourceOnBattlefield durations exist, and the source
//! sacrifices itself).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashnod's Transmogrant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice this artifact: Put a +1/+1 counter on target nonartifact creature. That creature becomes an artifact in addition to its other types.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().without_types(TypeLine::ARTIFACT.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: counter_and_artifactize,
        }),
    )
}

fn counter_and_artifactize(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "That creature becomes an artifact in addition to its other types"
    // is a PERMANENT type change — Effect::AddType only offers
    // Duration::EndOfTurn / WhileSourceOnBattlefield (and this source
    // sacrifices itself as a cost), so the type change is omitted.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
