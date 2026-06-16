//! Frodo, Determined Hero — `{1}{W}` 2/2 Legendary Halfling Warrior.
//! "Whenever Frodo enters or attacks, you may attach target Equipment you
//! control with mana value 2 or 3 to Frodo. During your turn, prevent all
//! damage that would be dealt to Frodo."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

fn equipment_target(reg: &mut CardRegistry) -> TargetRequirement {
    let equipment = reg.interner_mut().intern("Equipment");
    TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent()
                .with_subtype_sym(equipment)
                .controlled_by(ControllerConstraint::You)
                .with_min_cmc(2)
                .with_max_cmc(3),
        ),
        count: TargetCount::UpTo(1),
        controller: None,
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frodo, Determined Hero");
    let halfling = reg.interner_mut().intern("Halfling");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(warrior);

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

    let etb_req = equipment_target(reg);
    let attack_req = equipment_target(reg);

    // GAP: static "During your turn, prevent all damage that would be dealt to
    // Frodo" — a self-directed, controller-turn-gated prevention static is not
    // expressible with the available surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: attach_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![etb_req],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attach_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![attack_req],
            }),
    )
}

fn attach_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Attach {
        equipment_or_aura: *id,
        target: trig.source,
    }]
}
