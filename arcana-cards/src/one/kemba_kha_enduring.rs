//! Kemba, Kha Enduring — `{1}{W}` 2/2 Legendary Creature — Cat Cleric.
//!
//! Oracle:
//! * Whenever Kemba or another Cat you control enters, attach up to one target
//!   Equipment you control to that creature.
//! * Equipped creatures you control get +1/+1. (GAP: static anthem on equipped
//!   creatures; no continuous-static hook on this card class.)
//! * {3}{W}{W}: Create a 2/2 white Cat creature token.
//!
//! Decomposition: the Cat-enters attach → one `TriggeredAbilityDef` (targets
//! an Equipment you control, attaches it to the entering creature); the
//! token-maker → one `ActivatedAbilityDef`.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kemba, Kha Enduring");
    let cat = reg.interner_mut().intern("Cat");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(cleric);
    // Intern "Cat" again is fine via lookup later for the token name.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: script::subtype_filter(reg, "Cat")
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: attach_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        script::subtype_filter(reg, "Equipment")
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // GAP: "Equipped creatures you control get +1/+1." — static anthem.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}{W}: Create a 2/2 white Cat creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_cat,
            }),
    )
}

fn attach_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(creature) = trig.entering_object() else {
        return Vec::new();
    };
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(equip) = target else {
        return Vec::new();
    };
    vec![Effect::Attach {
        equipment_or_aura: *equip,
        target: creature,
    }]
}

fn make_cat(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(cat) = reg.interner().lookup("Cat") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: cat,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
