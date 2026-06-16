//! Captain America, First Avenger — `{R}{W}{U}` 4/4 Legendary Human
//! Soldier Hero.
//! Throw ... — "{3}, Unattach an Equipment from Captain America: He deals
//!   damage equal to that Equipment's mana value divided as you choose
//!   among one, two, or three targets." — GAP: the "Unattach an Equipment"
//!   activation cost and the "equal to that Equipment's mana value" amount
//!   are not expressible.
//! ... Catch — "At the beginning of combat on your turn, attach up to one
//!   target Equipment you control to Captain America."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Captain America, First Avenger");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let hero = reg.interner_mut().intern("Hero");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{U}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let equip_filter = script::subtype_filter(reg, "Equipment")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "Throw ..." — "Unattach an Equipment from Captain America"
            // is not an ActivationCost field, and the damage amount
            // ("equal to that Equipment's mana value") is not computable.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: catch_attach_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(equip_filter),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn catch_attach_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Attach { equipment_or_aura: *id, target: trig.source }]
}
