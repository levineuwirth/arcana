//! Diamond Faerie — `{2}{G}{W}{U}` 3/3 Snow Creature — Faerie.
//!
//! * Flying.
//! * `{1}{S}`: Snow creatures you control get +1/+1 until end of turn.
//!
//! Flying is a base keyword; the card is a Snow permanent (SNOW supertype).
//! The activated ability pumps every snow creature you control — computed at
//! resolution via `script::ids_matching` over a snow-creature filter, one
//! `Effect::Pump` per id wrapped in a `Sequence`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Diamond Faerie");
    let faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{S}: Snow creatures you control get +1/+1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_snow_creatures,
            }),
    )
}

/// `{1}{S}`: +1/+1 to each snow creature you control until end of turn.
fn pump_snow_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::SNOW));
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::Sequence(
        ids.into_iter()
            .map(|id| Effect::Pump {
                target: id,
                power: 1,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            })
            .collect(),
    )]
}
