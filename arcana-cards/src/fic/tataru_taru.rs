//! Tataru Taru — `{1}{W}` 0/3 Legendary Dwarf Advisor.
//!
//! * "When Tataru Taru enters, you draw a card and target opponent may draw a
//!   card." — the controller's draw is implemented; the "target opponent may
//!   draw" half is a free optional draw with no expressible shape, GAP'd.
//! * "Scions' Secretary — Whenever an opponent draws a card, if it isn't that
//!   player's turn, create a tapped Treasure token. (Once each turn.)" — the
//!   "if it isn't that player's turn" intervening-if is not expressible; the
//!   whole trigger is GAP'd rather than fired unconditionally.

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
    let name = reg.interner_mut().intern("Tataru Taru");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "target opponent may draw a card" — free optional draw not modeled.
    // GAP: "Scions' Secretary" opponent-draw trigger — "if it isn't that
    // player's turn" intervening-if not expressible.
}

fn etb_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
