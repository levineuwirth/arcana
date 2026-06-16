//! Old Man of the Sea — `{1}{U}{U}` 2/3 blue Djinn.
//! "You may choose not to untap this creature during your untap step."
//! "{T}: Gain control of target creature with power less than or equal to this
//! creature's power for as long as this creature remains tapped and that
//! creature's power remains less than or equal to this creature's power."
//!
//! The "may not untap" line is a pure static replacement, GAP'd. The tap
//! activation is modeled with `Effect::ChangeControl` (gain control of the
//! target creature); the dynamic power-≤-source restriction on the target and
//! the conditional "for as long as …" duration are not expressible — best
//! effort takes permanent control of any target creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Old Man of the Sea");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "you may choose not to untap this during your untap step" —
    //      untap-skip replacement, not expressible here.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Gain control of target creature with power less than or equal to this creature's power.".into(),
            cost: ActivationCost::tap_only(),
            // GAP: target restriction "power <= this creature's power" is a
            //      dynamic source-relative filter, not expressible; uses
            //      target creature.
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
    // GAP: faithful duration is "for as long as this remains tapped and that
    //      creature's power stays <= this creature's power"; ChangeControl is
    //      permanent.
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}
