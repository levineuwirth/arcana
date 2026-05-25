//! Thundercloud Shaman — `{3}{R}{R}` 4/4 red Giant Shaman.
//! "When this creature enters, it deals damage equal to the number of Giants
//! you control to each non-Giant creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    let name = reg.interner_mut().intern("Thundercloud Shaman");
    let giant = reg.interner_mut().intern("Giant");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: deal_damage_to_non_giants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn deal_damage_to_non_giants(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let giant_filter = script::subtype_filter(reg, "Giant")
        .controlled_by(ControllerConstraint::You);
    let giant_count = script::count_matching(state, &giant_filter, trig.controller);
    if giant_count == 0 {
        return Vec::new();
    }
    let non_giant_filter = script::subtype_filter(reg, "Giant");
    // We want non-Giants: all creatures that are NOT Giants.
    // Use ObjectFilter::creature() as the broadest; GAP: no NOT-subtype filter.
    // Approximate: deal to all creatures (GAP: cannot exclude Giants).
    let ids = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: giant_count,
            source: trig.source,
        }),
    }]
}
