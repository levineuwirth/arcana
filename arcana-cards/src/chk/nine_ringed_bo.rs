//! Nine-Ringed Bo — `{3}` artifact.
//! "{T}: This artifact deals 1 damage to target Spirit creature. If that
//! creature would die this turn, exile it instead."
//! Tap-activated ping at a Spirit creature; the dies-this-turn
//! exile-instead replacement is a documented gap.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Nine-Ringed Bo");
    let spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: This artifact deals 1 damage to target Spirit \
                   creature. If that creature would die this turn, exile it \
                   instead."
                .into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types(TypeLine::CREATURE.into())
                        .with_subtypes_any(vec![spirit]),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: ping_spirit,
        }),
    )
}

fn ping_spirit(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "If that creature would die this turn, exile it instead" — the
    // dies-to-exile replacement effect is not expressible.
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount: 1,
        source: ctx.source,
    }]
}
