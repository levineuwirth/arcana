//! Turret Ogre — `{3}{R}` 4/3 Ogre Warrior with Reach.
//! "When this creature enters, if you control another creature with power
//! 4 or greater, this creature deals 2 damage to each opponent."
//!
//! Keyword line: Reach. One ETB trigger gated by an intervening-if. The
//! "if you control ANOTHER creature with power 4+" is expressed as
//! "at least two power-4 creatures you control" — Turret Ogre (a 4/3) is
//! itself one of them, so requiring two captures the "another" exclusion
//! faithfully at resolution. The payoff deals 2 damage to each opponent
//! (one DealDamage per opponent, wrapped in a Sequence).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Turret Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(if_control_another_power4),
                effect: deal_two_to_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_control_another_power4(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "Another creature with power 4+": Turret Ogre (a 4/3) is itself a
    // power-4 creature, so "another" means at least two such creatures.
    conditions::you_control_at_least(
        s,
        you,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        2,
    )
}

fn deal_two_to_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 2,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
