//! Wick's Patrol — `{4}{B}{B}` 5/3 black Rat Warlock.
//! "When this creature enters, mill three cards. When you do, target creature
//! an opponent controls gets -X/-X until end of turn, where X is the greatest
//! mana value among cards in your graveyard."
//!
//! GAP: "greatest mana value among cards in your graveyard" — no script helper
//! exists for max CMC in graveyard; only graveyard_size is available. The
//! -X/-X pump effect is omitted entirely as the X value is not computable.
//! Emitting only the mill-3 half.

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
    let name = reg.interner_mut().intern("Wick's Patrol");
    let rat = reg.interner_mut().intern("Rat");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_and_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_mill_and_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Mill 3; the -X/-X rider requires greatest MV in graveyard which is
    // GAP: no script::max_cmc_in_graveyard helper available.
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}
