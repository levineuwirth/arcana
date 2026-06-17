//! Slinn Voda, the Rising Deep — `{6}{U}{U}` 8/8 Legendary Leviathan.
//! Kicker {1}{U}.
//! When Slinn Voda enters, if it was kicked, return all creatures to
//! their owners' hands except for Merfolk, Krakens, Leviathans,
//! Octopuses, and Serpents.
//!
//! Kicker is not in the usable keyword surface, and there is no
//! "was kicked" intervening-if helper / accessor to gate the ETB. The
//! mass bounce alone is expressible, but firing it unconditionally
//! (without the kicked gate) would be a materially wrong card, so the
//! whole ETB is GAP'd. Only the bones are emitted.

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
    let name = reg.interner_mut().intern("Slinn Voda, the Rising Deep");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);

    // GAP: Kicker {1}{U} — not in the usable keyword surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_kicked_bounce,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_kicked_bounce(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if it was kicked, return all creatures except [subtypes]" —
    // no "was kicked" intervening-if helper to gate the bounce.
    Vec::new()
}
