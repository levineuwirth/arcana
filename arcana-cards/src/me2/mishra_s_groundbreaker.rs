//! Mishra's Groundbreaker — {4} artifact (Antiquities, 1994).
//! "{T}, Sacrifice this artifact: Target land becomes a 3/3 artifact
//! creature that's still a land. (This effect lasts indefinitely.)"
//! Animates the target land; the indefinite duration is approximated
//! with end-of-turn (no indefinite Duration in this API).

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
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mishra's Groundbreaker");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Sacrifice this artifact: Target land becomes a \
                       3/3 artifact creature that's still a land."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: animate_land,
            },
        ),
    )
}

fn animate_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "This effect lasts indefinitely" — only EndOfTurn /
    // WhileSourceOnBattlefield durations exist, and the source is
    // sacrificed as a cost; EndOfTurn is the closest approximation.
    vec![
        Effect::AddType {
            target: *id,
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: *id,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        },
    ]
}
