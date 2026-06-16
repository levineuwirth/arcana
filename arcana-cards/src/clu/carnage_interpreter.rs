//! Carnage Interpreter — `{1}{B/R}{B/R}` 3/3 Devil Detective.
//! "When this creature enters, discard your hand, then investigate four
//!  times."
//! "As long as you have one or fewer cards in hand, this creature gets
//!  +2/+2 and has menace."
//!
//! The ETB (discard hand + investigate ×4) is wired. The hand-size
//! conditional +2/+2 / menace static has no primitive (GAP).

use arcana_core::effects::{CommodityToken, Effect, DiscardChoice};
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
    let name = reg.interner_mut().intern("Carnage Interpreter");
    let devil = reg.interner_mut().intern("Devil");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    subtypes.0.insert(detective);

    // GAP: "As long as you have one or fewer cards in hand, this creature
    //   gets +2/+2 and has menace" — conditional static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: discard_hand_investigate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn discard_hand_investigate(
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
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Clue,
            count: 4,
        },
    ])]
}
