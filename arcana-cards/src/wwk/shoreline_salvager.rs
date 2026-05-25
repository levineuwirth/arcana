//! Shoreline Salvager — `{3}{B}` 3/3 black Surrakar.
//! "Whenever this creature deals combat damage to a player, if you
//! control an Island, you may draw a card."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shoreline Salvager");
    let surrakar = reg.interner_mut().intern("Surrakar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(surrakar);
    // intern Island subtype at register time
    reg.interner_mut().intern("Island");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: damage_draw_if_island,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damage_draw_if_island(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // intervening_if: "if you control an Island"
    let island_filter = script::subtype_filter(reg, "Island")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &island_filter, trig.controller);
    if n > 0 {
        vec![Effect::DrawCards { player: trig.controller, count: 1 }]
    } else {
        Vec::new()
    }
}
