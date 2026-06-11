//! March from the Black Gate — `{1}{B}` enchantment.
//! "When this enchantment enters and whenever an Army you control
//! attacks, amass Orcs 1."
//!
//! Two triggers (ETB + filtered CreatureAttacks on Armies you control)
//! sharing one effect: `Effect::Amass` with race "Orc".

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("March from the Black Gate");
    let army = reg.interner_mut().intern("Army");
    // Pre-intern the race subtype for the resolver's read-only lookup.
    let _orc = reg.interner_mut().intern("Orc");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: amass_orcs_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .with_subtypes_any(vec![army])
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: amass_orcs_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…amass Orcs 1."
fn amass_orcs_one(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Orc").unwrap_or_default();
    vec![Effect::Amass {
        controller: trig.controller,
        count: 1,
        army_subtype,
        race_subtype,
    }]
}
