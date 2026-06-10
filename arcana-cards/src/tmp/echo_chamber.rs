//! Echo Chamber — `{4}` artifact (Tempest, 1997).
//! "{4}, {T}: An opponent chooses target creature they control. Create
//! a token that's a copy of that creature. That token gains haste until
//! end of turn. Exile the token at the beginning of the next end step.
//! Activate only as a sorcery."
//! The token copy is wired via `Effect::CopyPermanent`; the haste grant
//! and the delayed exile apply to the freshly minted token, whose id is
//! not visible to the resolver — both riders are documented GAPs. The
//! "an opponent chooses" wrinkle is approximated as a normal target
//! restricted to creatures an opponent controls.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Echo Chamber");
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
                text: "{4}, {T}: An opponent chooses target creature they \
                       control. Create a token that's a copy of that \
                       creature. That token gains haste until end of turn. \
                       Exile the token at the beginning of the next end \
                       step. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_creature,
            },
        ),
    )
}

fn copy_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "an opponent chooses target creature they control" — the
    // opponent's choice is approximated as a controller-chosen target.
    // GAP: "that token gains haste until end of turn" and "exile the
    // token at the beginning of the next end step" — the new token's id
    // is not visible to this resolver, so the riders cannot be attached.
    vec![Effect::CopyPermanent { target: *id }]
}
