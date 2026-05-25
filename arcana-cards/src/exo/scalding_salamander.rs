//! Scalding Salamander — `{2}{R}` 2/1 red Salamander creature.
//! "Whenever this creature attacks, you may have it deal 1 damage to each creature
//! without flying defending player controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Scalding Salamander");
    let salamander = reg.interner_mut().intern("Salamander");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(salamander);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_deal_to_grounders,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_deal_to_grounders(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let defender = match trig.defending_player() {
        Some(p) => p,
        None => return Vec::new(),
    };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    ids.into_iter().map(|id| Effect::DealDamage {
        target: DamageTarget::Object(id),
        amount: 1,
        source: trig.source,
    }).collect()
}
