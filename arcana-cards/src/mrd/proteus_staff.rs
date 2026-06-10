//! Proteus Staff — `{3}` artifact.
//! "{2}{U}, {T}: Put target creature on the bottom of its owner's
//! library. That creature's controller reveals cards from the top of
//! their library until they reveal a creature card. The player puts
//! that card onto the battlefield and the rest on the bottom of their
//! library in any order. Activate only as a sorcery."
//!
//! Wired as `PutOnBottomOfLibrary` on the target followed by
//! `RevealUntil` (found creature to the battlefield, rest to the
//! bottom — `DigRest::BottomRandom` approximates "in any order") for
//! the target's controller.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Proteus Staff");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}, {T}: Put target creature on the bottom of its owner's library. That creature's controller reveals cards from the top of their library until they reveal a creature card. The player puts that card onto the battlefield and the rest on the bottom of their library in any order. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: polymorph,
        }),
    )
}

fn polymorph(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let controller = script::target_controller(state, *id, ctx.controller);
    vec![
        Effect::PutOnBottomOfLibrary { target: *id },
        Effect::RevealUntil {
            player: controller,
            filter: ObjectFilter::creature(),
            found_dest: RevealDest::Battlefield,
            rest: DigRest::BottomRandom,
            max_reveal: None,
        },
    ]
}
