//! Undead Warchief — `{2}{B}{B}` 1/1 Zombie.
//! "Zombie spells you cast cost {1} less to cast. Zombie creatures you control
//! get +2/+1."
//!
//! * Zombie spells you cast cost {1} less.
//!   // GAP: spell cost reduction is not expressible on this card class.
//! * Zombie creatures you control get +2/+1.
//!   Wired via a SelfEntersBattlefield trigger installing a
//!   `ContinuousEffect::filtered_pump` over Zombie creatures you control,
//!   +2/+1, lasting while this creature is on the battlefield.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undead Warchief");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "Zombie spells you cast cost {1} less to cast" — spell cost
    // reduction is not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            // "Zombie creatures you control get +2/+1."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_zombie_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_zombie_anthem(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned at register");
    let filter = ObjectFilter::creature()
        .with_subtype_sym(zombie)
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            2,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
