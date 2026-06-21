//! Rubinia Soulsinger — `{2}{G}{W}{U}` 2/3 Legendary Faerie.
//!
//! Oracle:
//! * "You may choose not to untap Rubinia Soulsinger during your untap step." —
//!   a static untap-step restriction with no expressible primitive. GAP'd.
//! * "{T}: Gain control of target creature for as long as you control Rubinia
//!   Soulsinger and Rubinia Soulsinger remains tapped." — a tap-activated
//!   gain-control of a target creature. Effect::ChangeControl is the closest
//!   primitive; the "for as long as ~ remains tapped" conditional duration is a
//!   fidelity gap (modeled as a lasting control change).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rubinia Soulsinger");
    let faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "You may choose not to untap Rubinia Soulsinger during your
    // untap step." — no untap-step opt-out primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Gain control of target creature for as long as you \
                       control Rubinia Soulsinger and Rubinia Soulsinger remains \
                       tapped.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_control,
            }),
    )
}

fn gain_control(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Fidelity gap: "for as long as ~ remains tapped" conditional duration is
    // modeled as a lasting control change.
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}
