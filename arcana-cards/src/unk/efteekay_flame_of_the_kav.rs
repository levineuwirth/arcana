//! Efteekay, Flame of the Kav — `{4}{R}{G}` 4/2 Legendary Kavu Soldier.
//!
//! Eminence — As long as Efteekay is in the command zone or on the
//! battlefield, other Kavu spells you cast cost {1} less to cast.
//! Whenever Efteekay or another Kavu you control enters, it deals damage
//! equal to its power to target creature.
//!
//! GAP: the Eminence static cost-reduction has no triggered/activated
//! representation in this card class (no cost-reduction effect surface) —
//! omitted. The ETB-damage trigger is fully wired.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Efteekay, Flame of the Kav");
    let kavu = reg.interner_mut().intern("Kavu");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);
    subtypes.0.insert(soldier);
    let kavu_filter = script::subtype_filter(reg, "Kavu").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: kavu_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: kavu_etb_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn kavu_etb_damage(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "it" = the Kavu that entered; "equal to its power".
    let source = trig.entering_object().unwrap_or(trig.source);
    let amount = script::power_of(state, source).max(0) as u32;
    vec![Effect::DealDamage {
        source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
