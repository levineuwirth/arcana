//! Arrogant Outlaw — `{2}{B}` 3/2 black Vampire Noble.
//! "When this creature enters, if an opponent lost life this turn, each
//! opponent loses 2 life and you gain 2 life."
//! Intervening-if "if an opponent lost life this turn" wired via
//! `conditions::an_opponent_lost_life_this_turn`.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arrogant Outlaw");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
                // Intervening-if "if an opponent lost life this turn" via
                // conditions::an_opponent_lost_life_this_turn.
                intervening_if: Some(iif_opponent_lost_life),
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_opponent_lost_life(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::an_opponent_lost_life_this_turn(state, you)
}

fn on_etb(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let mut effects: Vec<Effect> = opponents
        .iter()
        .map(|&p| Effect::LoseLife { player: p, amount: 2 })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    effects
}
