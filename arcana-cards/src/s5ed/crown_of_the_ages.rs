//! Crown of the Ages — `{2}` artifact (Ice Age, 1995).
//! "{4}, {T}: Attach target Aura attached to a creature to another
//! creature."
//! Wired with two targets — the Aura and the destination creature —
//! and `Effect::Attach`. The "attached to a creature" precondition on
//! the Aura target and the "another creature" exclusion are not
//! expressible in an `ObjectFilter` and are documented GAPs.

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
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crown of the Ages");
    let aura = reg.interner_mut().intern("Aura");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Attach target Aura attached to a creature \
                       to another creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    // GAP: "attached to a creature" — attachment-state
                    // predicates are not expressible in an ObjectFilter.
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .with_types(TypeLine::ENCHANTMENT.into())
                                .with_subtypes_any(vec![aura]),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    // GAP: "another creature" — cannot exclude the Aura's
                    // current host from the second target's filter.
                    TargetRequirement::target_creature(),
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: move_aura,
            },
        ),
    )
}

fn move_aura(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(aura)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(creature)) = ctx.targets.targets.get(1)
    else {
        return Vec::new();
    };
    vec![Effect::Attach { equipment_or_aura: *aura, target: *creature }]
}
