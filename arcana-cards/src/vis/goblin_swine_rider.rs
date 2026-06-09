//! Goblin Swine-Rider — `{R}` 1/1 red Goblin. "Whenever this creature
//! becomes blocked, it deals 2 damage to each attacking creature and
//! each blocking creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Swine-Rider");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: deal_to_combatants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn deal_to_combatants(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // 2 damage to each attacking creature and each blocking creature.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().attacking_or_blocking_only(),
        trig.controller,
    );
    ids.into_iter()
        .map(|id| Effect::DealDamage {
            target: DamageTarget::Object(id),
            amount: 2,
            source: trig.source,
        })
        .collect()
}
