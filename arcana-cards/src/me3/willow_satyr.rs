//! Willow Satyr — `{2}{G}{G}` 1/1 Satyr.
//!
//! Oracle:
//! * "You may choose not to untap this creature during your untap
//!   step." — GAP: a may-not-untap static (no triggered/activated form
//!   to express the untap-restriction choice).
//! * "{T}: Gain control of target legendary creature for as long as
//!   you control this creature and this creature remains tapped." — a
//!   tap activation that gains control of a legendary creature. The
//!   "for as long as … remains tapped" conditional duration is a
//!   fidelity partial (Effect::ChangeControl is a permanent
//!   gain-control; no conditional-duration linkage is expressible).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Willow Satyr");
    let satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Gain control of target legendary creature for as long as you control this creature and this creature remains tapped.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_control,
        }),
    )
}

fn gain_control(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}
