//! Papercraft Decoy — `{2}` 2/1 colorless Artifact Creature — Frog.
//! "When this creature leaves the battlefield, you may pay {2}. If you do, draw a card."
//! GAP: "leaves the battlefield" trigger not in the engine trigger condition catalog;
//! using SelfDies as closest (fires on death only, not on other zone changes).
//! GAP: optional mana payment cost not expressible; drawing unconditionally as best-effort.

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
    let name = reg.interner_mut().intern("Papercraft Decoy");
    let frog = reg.interner_mut().intern("Frog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "leaves the battlefield" trigger not in catalog; using SelfDies
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_leaves_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_leaves_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional {2} payment not expressible; drawing unconditionally as best-effort
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
