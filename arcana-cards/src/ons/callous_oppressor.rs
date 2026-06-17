//! Callous Oppressor — `{1}{U}{U}` 1/2 Octopus.
//! "You may choose not to untap this creature during your untap step." (GAP'd)
//! "As this creature enters, an opponent chooses a creature type." (GAP'd)
//! "{T}: Gain control of target creature that isn't of the chosen type for as
//! long as this creature remains tapped."

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
    let name = reg.interner_mut().intern("Callous Oppressor");
    let octopus = reg.interner_mut().intern("Octopus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);

    // GAP (static): "may choose not to untap during your untap step" — no
    // untap-restriction primitive available.
    // GAP (replacement): "as this creature enters, an opponent chooses a
    // creature type" — no choose-a-type-on-entry primitive available; the
    // "isn't of the chosen type" target restriction is therefore also unmodeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Gain control of target creature that isn't of the chosen type for as long as this creature remains tapped.".into(),
            cost: ActivationCost { tap: true, ..ActivationCost::default() },
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

fn gain_control(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP (duration): "for as long as this creature remains tapped" — modeled
    // as permanent control change (no tapped-linked control duration primitive).
    vec![Effect::ChangeControl { target: *id, new_controller: ctx.controller }]
}
