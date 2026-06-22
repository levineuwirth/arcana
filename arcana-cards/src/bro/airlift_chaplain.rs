//! Airlift Chaplain — `{2}{W}` 1/1 white Human Cleric with Flying.
//!
//! Oracle:
//! * Flying.
//! * When this creature enters, mill three cards. You may put a Plains card
//!   or a creature card with mana value 3 or less from among the cards
//!   milled this way into your hand. If you don't, put a +1/+1 counter on
//!   this creature.
//!
//! Flying is on the keyword line. The ETB mills three cards.
//!
//! GAP: "You may put a Plains card or a creature card with mana value 3 or
//! less from among the cards milled this way into your hand. If you don't,
//! put a +1/+1 counter on this creature." — selecting a card from the
//! transient set of cards milled THIS WAY (a graveyard subset filtered by
//! Plains-or-cheap-creature), and the conditional +1/+1 counter on
//! decline, is not expressible with the documented effect surface. Only
//! the mill is emitted.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Airlift Chaplain");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_mill_three,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_mill_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}
