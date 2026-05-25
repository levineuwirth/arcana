//! Storm Caller — `{2}{R}` 3/2 red Ogre Shaman. "When this
//! creature enters, it deals 2 damage to each opponent." ETB
//! trigger that enumerates opponents at resolution and emits one
//! `DealDamage` per opponent.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Storm Caller");
    let ogre = reg.interner_mut().intern("Ogre");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: deal 2 damage to each opponent — enumerate
/// opponents from the live state and emit one `DealDamage` per
/// player, wrapped in a `Sequence`.
fn etb_damage_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, trig.controller);
    let damages: Vec<Effect> = opps
        .into_iter()
        .map(|p| Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 2,
            source: trig.source,
        })
        .collect();
    vec![Effect::Sequence(damages)]
}
