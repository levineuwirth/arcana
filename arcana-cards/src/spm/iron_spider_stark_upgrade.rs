//! Iron Spider, Stark Upgrade — `{3}` 2/3 Legendary Artifact Creature —
//! Spider Hero, with Vigilance.
//! "{T}: Put a +1/+1 counter on each artifact creature and/or Vehicle
//! you control."
//! "{2}, Remove two +1/+1 counters from among artifacts you control:
//! Draw a card."
//!
//! Vigilance is a base keyword. The first activation is a ForEach
//! +1/+1 over artifact creatures and Vehicles you control. The second
//! activation's "remove two +1/+1 counters from among artifacts you
//! control" cost is not expressible (remove_self_counter only removes
//! from this card), so that part of the cost is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iron Spider, Stark Upgrade");
    let spider = reg.interner_mut().intern("Spider");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Put a +1/+1 counter on each artifact creature and/or Vehicle you control.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_each_artifact_creature_or_vehicle,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Remove two +1/+1 counters from among artifacts you control"
                // is not expressible as an ActivationCost (remove_self_counter
                // removes from THIS card only); only the {2} mana cost is modeled.
                text: "{2}, Remove two +1/+1 counters from among artifacts you control: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_a_card,
            }),
    )
}

fn counter_each_artifact_creature_or_vehicle(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let artifact_creatures = ObjectFilter::creature()
        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
        .controlled_by(ControllerConstraint::You);
    let vehicles =
        script::subtype_filter(reg, "Vehicle").controlled_by(ControllerConstraint::You);

    let mut ids = script::ids_matching(state, &artifact_creatures, ctx.controller);
    for id in script::ids_matching(state, &vehicles, ctx.controller) {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }

    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}

fn draw_a_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
