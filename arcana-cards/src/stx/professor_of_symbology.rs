//! Professor of Symbology — `{1}{W}` 2/1 white Kor Cleric. "When this
//! creature enters, learn. (You may reveal a Lesson card you own from
//! outside the game and put it into your hand, or discard a card to
//! draw a card.)"
//!
//! Learn is not a modeled mechanic: the sideboard "reveal a Lesson
//! from outside the game" branch has no catalog effect, and the choice
//! between the two branches isn't expressible. Emitted as a best-effort
//! ETB rendering the second branch (discard a card, then draw a card),
//! which the discard/draw primitives can express; the Lesson-retrieval
//! branch and the player's choice are the gap.

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Professor of Symbology");
    let kor = reg.interner_mut().intern("Kor");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_learn,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_learn(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Learn is unmodeled — the "reveal a Lesson card from outside
    // the game" branch and the player's choice between branches are not
    // expressible. Best-effort: the discard-a-card-then-draw-a-card branch.
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ])]
}
