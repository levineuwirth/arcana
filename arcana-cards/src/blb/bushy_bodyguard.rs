//! Bushy Bodyguard — `{1}{G}` 2/1 Squirrel Warrior.
//! Offspring {2}; "When this creature enters, you may forage. If you do,
//! put two +1/+1 counters on it."
//!
//! GAP: Offspring is a cast-time additional-cost keyword (pay {2} → 1/1 token
//!      copy on ETB) — not expressible with the available keyword/effect surface.
//! GAP: Forage (exile three cards from your graveyard OR sacrifice a Food) is a
//!      composite alternate cost not expressible as an OptionalPaymentKind; the
//!      "if you do, put two +1/+1 counters" payload is therefore gapped.

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
    let name = reg.interner_mut().intern("Bushy Bodyguard");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Forage / Offspring keywords not in the available keyword surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_forage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_forage(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may forage. If you do, put two +1/+1 counters on it." — Forage is a
    // composite alternate cost (exile three cards from graveyard OR sacrifice a Food)
    // not expressible as an OptionalPaymentKind, so the gated payload is omitted.
    Vec::new()
}
