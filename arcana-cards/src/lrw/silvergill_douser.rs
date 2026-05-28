//! Silvergill Douser — `{1}{U}` 1/1 blue Merfolk Wizard.
//! "{T}: Target creature gets -X/-0 until end of turn, where X is the number of
//! Merfolk and/or Faeries you control."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silvergill Douser");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let _faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature gets -X/-0 until end of turn, X = Merfolk and/or Faeries you control.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: debuff_by_merfolk_count,
            }),
    )
}

fn debuff_by_merfolk_count(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let merfolk_filter = script::subtype_filter(reg, "Merfolk")
        .controlled_by(ControllerConstraint::You);
    let faerie_filter = script::subtype_filter(reg, "Faerie")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &merfolk_filter, ctx.controller)
        + script::count_matching(state, &faerie_filter, ctx.controller);
    vec![Effect::Pump {
        target: *id,
        power: -(n as i32),
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
