//! Putrid Cyclops — `{2}{B}` 3/3 black Zombie Cyclops. "When this
//! creature enters, scry 1, then reveal the top card of your library.
//! This creature gets -X/-X until end of turn, where X is that card's
//! mana value."
//!
//! The scry 1 is expressible; the "reveal top card and shrink by its
//! mana value" rider derives a dynamic amount from the revealed
//! library card — there is no script helper that reads the top
//! library card's mana value, and "reveal, then -X/-X where X = that
//! card's mana value" is not expressible with the catalog. The scry
//! is emitted; the dynamic self-debuff is GAP-ed.

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
    let name = reg.interner_mut().intern("Putrid Cyclops");
    let zombie = reg.interner_mut().intern("Zombie");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cyclops);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scry_then_shrink,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_scry_then_shrink(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top card of your library; this creature gets
    // -X/-X where X is that card's mana value" — no script helper
    // reads the top library card's mana value, and the reveal-then-
    // derive-X self-debuff is not expressible. Only the scry is emitted.
    vec![Effect::Scry { player: trig.controller, count: 1 }]
}
