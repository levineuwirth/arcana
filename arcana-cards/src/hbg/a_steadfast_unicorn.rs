//! A-Steadfast Unicorn — `{W}` 1/2 Unicorn.
//! `{4}{W}: Creatures you control get +1/+1 and gain vigilance until end of turn.
//! Activate only during your turn.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Steadfast Unicorn");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}: Creatures you control get +1/+1 and gain vigilance until end of turn. Activate only during your turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_all_vigilance,
            }),
    )
}

fn pump_all_vigilance(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Vigilance],
        })
        .collect()
}
