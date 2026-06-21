//! "Brims" Barone, Midway Mobster — `{3}{W}{B}` 5/4 Legendary Human Rogue.
//!
//! Oracle:
//! * When ~ enters, put a +1/+1 counter on each other creature you control
//!   that has a hat.
//! * ~ has menace as long as you're wearing a hat.
//!
//! Both abilities key off the Unfinity "hat" / "wearing a hat" attraction
//! mechanic, which has no engine representation (no "has a hat" object filter
//! and no "you're wearing a hat" condition). The ETB counter distribution
//! and the conditional menace are therefore GAP'd; only the bones survive.

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
    let name = reg.interner_mut().intern("\"Brims\" Barone, Midway Mobster");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    // GAP: static — "~ has menace as long as you're wearing a hat" (no "wearing a hat" condition).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_hat_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_hat_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/+1 counter on each other creature you control that has a hat" —
    // the "has a hat" object property is not representable in ObjectFilter.
    Vec::new()
}
