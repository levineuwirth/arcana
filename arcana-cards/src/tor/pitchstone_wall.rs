//! Pitchstone Wall — `{2}{R}` 2/5 Wall.
//! Defender.
//! "Whenever you discard a card, you may sacrifice this creature. If you
//! do, return the discarded card from your graveyard to your hand."
//!
//! GAP: the discard trigger's payload — "you may sacrifice this creature;
//! if you do, return THE DISCARDED CARD from your graveyard to your hand"
//! — needs (a) an optional sacrifice gate and (b) a handle to the specific
//! card just discarded. OptionalPayment only models Mana/Life costs (no
//! Sacrifice gate), and no PendingTrigger accessor surfaces the discarded
//! card's id. The trigger is wired (CardDiscarded by you) but its effect
//! is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pitchstone Wall");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDiscarded {
                player: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: on_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_discard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice this creature. If you do, return THE
    // DISCARDED CARD from your graveyard to your hand." No optional-
    // sacrifice gate (OptionalPayment is Mana/Life only) and no accessor
    // for the specific discarded card's id.
    Vec::new()
}
