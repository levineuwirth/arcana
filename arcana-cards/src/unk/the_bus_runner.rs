//! The Bus Runner — `{1}{B}{G}{U}` 3/4 Legendary Human Gamer.
//!
//! When The Bus Runner enters, create a 4/4 Desert Vehicle artifact land
//! token with crew 2, eight hour counters, a "{T}: Add {C}" ability, and a
//! self-tapping hour-counter loop that drains opponents.
//! Ready to run (commander-pairing rule).
//!
//! The ETB token carries its own activated and triggered abilities plus an
//! hour-counter countdown loop; `TokenDefinition` cannot express those
//! nested abilities or the counter bookkeeping, so emitting a bare token
//! would be materially wrong. The whole ETB effect is GAP'd, as is the
//! commander-only "Ready to run" static. The ETB trigger is still wired so
//! the shape is recorded.

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
    let name = reg.interner_mut().intern("The Bus Runner");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    // GAP: static — "Ready to run" is a commander-pairing rule, not a
    //      triggered/activated ability and unmodeled.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_create_bus_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_create_bus_token(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token has crew 2, eight hour counters, "{T}: Add {C}", and a
    // self-tapping hour-counter countdown that drains opponents and resets.
    // TokenDefinition cannot carry these nested activated/triggered abilities
    // or the named-counter loop, so the whole effect is declined.
    Vec::new()
}
