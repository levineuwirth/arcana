//! Shagrat, Loot Bearer — `{2}{B}{R}` 4/4 legendary black/red Orc Soldier.
//! "Whenever Shagrat attacks, attach up to one target Equipment to it.
//! Then amass Orcs X, where X is the number of Equipment attached to
//! Shagrat."
//!
//! # GAP
//! - Amass (Orcs X) is not in the engine effect catalog; no `Effect::Amass`
//!   variant exists. The amass half is omitted.
//! - "Number of Equipment attached to Shagrat" requires querying aura/equipment
//!   attachment state on a specific permanent, which has no `script::*` helper.
//!   The X computation is therefore not expressible.
//! - The attach effect is modeled using `Effect::Attach` with the Equipment as
//!   target and Shagrat as the destination, but the API shape is
//!   `Attach { equipment_or_aura, target }` where `equipment_or_aura` is the
//!   source (the Aura/Equipment) and `target` is the thing being attached to.
//!   Here the Equipment is the chosen target and Shagrat is the source, so we
//!   use `target: trig.source` as the creature receiving the Equipment.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shagrat, Loot Bearer");
    let orc = reg.interner_mut().intern("Orc");
    let soldier = reg.interner_mut().intern("Soldier");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // Pre-intern Equipment subtype for use in target filter
    let _ = equipment;
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attach_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine(TypeLine::ARTIFACT))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn attach_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // If no equipment was chosen (up to one), return nothing
    let Some(target) = trig.targets.targets.first() else {
        // GAP: amass Orcs X not expressible (no Effect::Amass variant)
        return Vec::new();
    };
    let TargetChoice::Object(equip_id) = target else {
        return Vec::new();
    };
    vec![
        Effect::Attach {
            equipment_or_aura: *equip_id,
            target: trig.source,
        },
        // GAP: amass Orcs X not expressible; X = number of Equipment attached
        // to Shagrat, which has no script::* helper. Omitted.
    ]
}
