//! Ertai's Familiar — `{1}{U}` 2/2 Creature — Illusion.
//!
//! * Phasing — NOT in the usable KeywordAbility surface; GAP'd (emit no keyword).
//! * When this creature phases out or leaves the battlefield, mill three cards.
//!   ("phases out" has no trigger; modeled via SelfLeavesBattlefield, which
//!   covers the leaves-battlefield half.)
//! * {U}: Until your next upkeep, this creature can't phase out. — no phasing
//!   primitive; GAP'd.

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
    let name = reg.interner_mut().intern("Ertai's Familiar");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Phasing / Mill keywords not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "phases out" half of the trigger is unmodeled; the
            // leaves-the-battlefield half is faithful.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: mill_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "{U}: Until your next upkeep, this creature can't phase out" —
    // no phasing primitive to gate.
}

fn mill_three(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}
