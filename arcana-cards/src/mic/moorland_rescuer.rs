//! Moorland Rescuer — `{5}{W}` 4/4 white Human Knight. "When this
//! creature dies, return any number of other creature cards with total
//! power X or less from your graveyard to the battlefield, where X is
//! this creature's power. Exile this card."
//!
//! The mass-reanimation clause ("return any number of other creature
//! cards with total power X or less ... to the battlefield") is not
//! expressible: the variable-count graveyard selection primitive
//! (`ChooseAnyNumberFromZone`) only supports the ReturnToHand /
//! Discard / Sacrifice / Exile actions, not return-to-battlefield, and
//! there is no power-budget ("total power X or less") gate. The
//! self-exile rider is modeled.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
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
    let name = reg.interner_mut().intern("Moorland Rescuer");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "return any number of other creature cards with total power X
    // or less from your graveyard to the battlefield, where X is this
    // creature's power" — no return-to-battlefield variant on the
    // variable-count graveyard selection, and no total-power budget gate.
    // The self-exile rider IS modeled.
    let id = trig.dying_object().unwrap_or(trig.source);
    vec![Effect::DelayedAction {
        source: id,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Exile,
    }]
}
