//! High Perfect Morcant — `{2}{B}{G}` 4/4 Legendary Elf Noble.
//! "Whenever High Perfect Morcant or another Elf you control enters, each
//! opponent blights 1."
//! "Tap three untapped Elves you control: Proliferate. Activate only as a sorcery."
//!
//! GAP: "each opponent blights 1" (each opponent puts a -1/-1 counter on a
//!      creature THEY control) has no corresponding Effect — there is no
//!      opponent-directed self-counter primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("High Perfect Morcant");
    let elf = reg.interner_mut().intern("Elf");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    let elf_trigger_filter = script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);
    let elf_tap_filter = script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: elf_trigger_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: blight_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap three untapped Elves you control: Proliferate. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(elf_tap_filter),
                    tap_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: proliferate,
            }),
    )
}

fn blight_each_opponent(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each opponent blights 1" — no Effect for an opponent placing a -1/-1
    // counter on a creature of their own choosing.
    Vec::new()
}

fn proliferate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Proliferate]
}
