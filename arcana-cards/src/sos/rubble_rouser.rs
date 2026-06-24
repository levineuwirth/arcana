//! Rubble Rouser — `{2}{R}` 1/4 Dwarf Sorcerer.
//! "When this creature enters, you may discard a card. If you do, draw a card."
//! "{T}, Exile a card from your graveyard: Add {R}. When you do, this
//!  creature deals 1 damage to each opponent."
//!
//! The ETB "you may discard a card; if you do, draw" is wired as an
//! OptionalPayment (cost = Discard(1), then = draw a card). The activated
//! ability is GAP'd: there is no "exile a card from your graveyard"
//! activation-cost field, and the reflexive "when you do, deal 1 to each
//! opponent" rider on a mana ability isn't expressible.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "{T}, Exile a card from your graveyard: Add {R}. When you do, this
// creature deals 1 damage to each opponent." No exile-from-graveyard cost
// field, and no reflexive-trigger rider on a mana ability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rubble Rouser");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_rummage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_rummage(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "You may discard a card. If you do, draw a card."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}
