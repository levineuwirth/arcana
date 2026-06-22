//! Ox of Agonas — `{3}{R}{R}` 4/2 Ox.
//! * "When this creature enters, discard your hand, then draw three cards."
//!   (triggered)
//! * Escape—{R}{R}, Exile eight other cards from your graveyard. (GAP)
//! * "This creature escapes with a +1/+1 counter on it." (part of Escape; GAP)
//!
//! GAP: Escape is not in the supported KeywordAbility surface, and its
//! graveyard-cast cost ("exile eight other cards") plus the escapes-with-a-
//! counter rider have no demonstrated primitive. The ETB trigger is wired:
//! "discard your hand" is modeled as a discard of the controller's whole hand
//! (count = current hand size), followed by drawing three cards.

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Ox of Agonas");
    let ox = reg.interner_mut().intern("Ox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ox);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_hand_draw_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_discard_hand_draw_three(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let hand = script::hand_size(state, trig.controller);
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: trig.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: trig.controller, count: 3 },
    ])]
}
