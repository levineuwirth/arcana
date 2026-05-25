//! Cyclone Summoner — `{5}{U}{U}` 7/7 blue creature. "When this creature
//! enters, if you cast it from your hand, return all permanents to their
//! owners' hands except for Giants, Wizards, and lands."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cyclone Summoner");
    let giant = reg.interner_mut().intern("Giant");
    let wizard = reg.interner_mut().intern("Wizard");
    let _giant2 = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // intervening_if: "if you cast it from your hand" — GAP: cast-from-hand check not supported
                intervening_if: None,
                effect: bounce_all,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn bounce_all(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Return all permanents except Giants, Wizards, and lands
    let giant_filter = script::subtype_filter(reg, "Giant");
    let wizard_filter = script::subtype_filter(reg, "Wizard");
    let all_permanents = script::ids_matching(
        state,
        &ObjectFilter::permanent().controlled_by(ControllerConstraint::Any),
        trig.controller,
    );
    let giant_ids = script::ids_matching(state, &giant_filter, trig.controller);
    let wizard_ids = script::ids_matching(state, &wizard_filter, trig.controller);
    let land_ids = script::ids_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::LAND.into()).controlled_by(ControllerConstraint::Any),
        trig.controller,
    );
    let excluded: std::collections::HashSet<_> = giant_ids.iter()
        .chain(wizard_ids.iter())
        .chain(land_ids.iter())
        .copied()
        .collect();
    let targets: Vec<_> = all_permanents.into_iter()
        .filter(|id| !excluded.contains(id))
        .collect();
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::ReturnToHand {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
