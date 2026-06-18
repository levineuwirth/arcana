//! Conifer Wurm — `{4}{G}` 4/4 Snow Creature — Wurm with Trample.
//! "{3}{G}: This creature gets +X/+X until end of turn, where X is the number
//! of snow permanents you control."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conifer Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G}: This creature gets +X/+X until end of turn, where X is the number of snow permanents you control.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_by_snow,
        }),
    )
}

fn pump_by_snow(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let snow_filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::SNOW));
    let n = script::count_matching(state, &snow_filter, ctx.controller) as i32;
    vec![Effect::Pump {
        target: ctx.source,
        power: n,
        toughness: n,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
