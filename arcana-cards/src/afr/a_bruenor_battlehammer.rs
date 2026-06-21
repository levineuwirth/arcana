//! A-Bruenor Battlehammer — `{2}{R}{W}` 5/4 Legendary Creature — Dwarf Warrior.
//! "Each creature you control gets +2/+0 for each Equipment attached to it.
//!  {0}: Attach target Equipment you control to target creature you control.
//!  Activate only as a sorcery and only once each turn."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Bruenor Battlehammer");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(warrior);

    let equipment_filter = script::subtype_filter(reg, "Equipment")
        .controlled_by(ControllerConstraint::You);

    // GAP (static): "Each creature you control gets +2/+0 for each Equipment
    // attached to it" — no documented per-attachment dynamic anthem static hook.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{0}: Attach target Equipment you control to target creature you control. Activate only as a sorcery and only once each turn.".into(),
            cost: ActivationCost {
                once_per_turn: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(equipment_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: attach_equipment,
        }),
    )
}

fn attach_equipment(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut targets = ctx.targets.targets.iter();
    let equipment_id = match targets.next() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let creature_id = match targets.next() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::Attach {
        equipment_or_aura: equipment_id,
        target: creature_id,
    }]
}
