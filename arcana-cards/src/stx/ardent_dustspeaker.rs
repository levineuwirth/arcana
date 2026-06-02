//! Ardent Dustspeaker — `{4}{R}` 3/4 Minotaur Shaman. "Whenever this
//! creature attacks, you may put an instant or sorcery card from your
//! graveyard on the bottom of your library. If you do, exile the top two
//! cards of your library. You may play those cards this turn."
//!
//! The attack trigger fires; we model the payoff with `Effect::ImpulseExile`
//! (exile the top two, playable this turn). The optional "put an instant or
//! sorcery from your graveyard on the bottom" precondition has no expressible
//! cost shape on a triggered ability, so it is GAP'd — the impulse is applied
//! unconditionally as a best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ardent Dustspeaker");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the optional "put an instant or sorcery card from your graveyard
    // on the bottom of your library" precondition ("if you do") has no
    // expressible cost/choice shape here; applying the impulse-exile payoff
    // unconditionally as a best effort.
    vec![Effect::ImpulseExile { player: trig.controller, count: 2 }]
}
