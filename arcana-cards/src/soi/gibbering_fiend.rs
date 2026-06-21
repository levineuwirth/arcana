//! Gibbering Fiend — `{1}{R}` 2/1 Devil.
//!
//! "When this creature enters, it deals 1 damage to each opponent.
//!  Delirium — At the beginning of each opponent's upkeep, if there are
//!  four or more card types among cards in your graveyard, this creature
//!  deals 1 damage to that player."
//!
//! ETB damage-to-each-opponent is expressed via a Sequence of per-opponent
//! DealDamage effects. The Delirium upkeep trigger is GAP'd: its
//! intervening-if ("four or more card types in your graveyard") has no
//! `conditions::` predicate, so the trigger cannot be gated correctly and
//! firing unconditionally would be a wrong card.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gibbering Fiend");
    let devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
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
        // GAP: "Delirium — At the beginning of each opponent's upkeep, if
        // there are four or more card types among cards in your graveyard,
        // deal 1 damage to that player." The intervening-if condition
        // (4+ card types in graveyard) is not expressible with the
        // available `conditions::` predicates, so the whole ability is
        // omitted (firing unconditionally would be incorrect).
    )
}

fn etb_damage_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
