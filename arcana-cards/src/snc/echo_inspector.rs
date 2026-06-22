//! Echo Inspector — `{3}{U}` 2/3 Creature — Bird Rogue.
//!
//! Oracle:
//! * Flying.
//! * When this creature enters, it connives. (Draw a card, then discard a
//!   card. If you discarded a nonland card, put a +1/+1 counter on this
//!   creature.)
//!
//! Decomposition: Flying keyword + one ETB trigger that connives. "Connive" is
//! an ability word, not a keyword.
//!
//! GAP: connive's "if you discarded a nonland card, put a +1/+1 counter on
//!      this creature" branch is conditional on the discarded card's type —
//!      not expressible. Best-effort emits the draw + discard (cf. Raffine's
//!      Informant).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Echo Inspector");
    let bird = reg.interner_mut().intern("Bird");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_connive,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_connive(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if you discarded a nonland card, put a +1/+1 counter on this
    //      creature" — conditional-on-discard-type, not expressible.
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
