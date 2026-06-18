//! Greatbow Doyen — `{4}{G}` 2/4 Elf Archer.
//!
//! "Other Archer creatures you control get +1/+1.
//!  Whenever an Archer you control deals damage to a creature, that Archer
//!  deals that much damage to that creature's controller."
//!
//! The anthem ("Other Archer creatures you control get +1/+1") is a static
//! continuous effect with no trigger/cost — GAP'd. The damage trigger fires
//! on an Archer you control dealing damage to a creature, but its payload
//! redirects "that much" damage from "that Archer" to "that creature's
//! controller" — there's no accessor for the damage source object or for the
//! damaged object's controller, so the payload is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Greatbow Doyen");
    let elf = reg.interner_mut().intern("Elf");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let archer_src = script::subtype_filter(reg, "Archer")
        .controlled_by(ControllerConstraint::You);

    // GAP (static): "Other Archer creatures you control get +1/+1."
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: archer_src,
                    target_filter: TargetFilter::Creature,
                    combat_only: false,
                },
                intervening_if: None,
                effect: archer_damage_to_controller,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn archer_damage_to_controller(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that Archer deals that much damage to that creature's controller"
    // — no accessor for the damage-source object or the damaged creature's
    // controller; only the amount is available, not the redirected targets.
    Vec::new()
}
