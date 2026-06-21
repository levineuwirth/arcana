//! Orzhov Pontiff — `{1}{W}{B}` 1/1 Creature — Human Cleric.
//!
//! Oracle:
//! * Haunt (When this creature dies, exile it haunting target creature.) — GAP'd
//! * When this creature enters OR the creature it haunts dies, choose one —
//!     • Creatures you control get +1/+1 until end of turn.
//!     • Creatures you don't control get -1/-1 until end of turn.
//!
//! Haunt is not in the keyword surface and its exile-haunting mechanic has
//! no primitive — GAP'd (this also removes the "creature it haunts dies"
//! alternate trigger). The ETB half of the trigger is wired structurally,
//! but its payload is a modal "choose one" — triggered abilities have no
//! modal-dispatch field (only spell abilities do), so the choice itself is
//! GAP'd and the effect body is empty.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orzhov Pontiff");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // GAP: Haunt — no KeywordAbility variant and no exile-haunting primitive;
    //      this also removes the "creature it haunts dies" alternate trigger.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: pontiff_choice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pontiff_choice(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "choose one — Creatures you control get +1/+1 / Creatures you don't
    //      control get -1/-1" — modal choice has no triggered-ability dispatch
    //      (modal is only on spell abilities); cannot post the mode selection.
    Vec::new()
}
