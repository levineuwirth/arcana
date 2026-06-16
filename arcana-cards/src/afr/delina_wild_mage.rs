//! Delina, Wild Mage — `{3}{R}` 3/2 Legendary Elf Shaman.
//! "Whenever Delina attacks, choose target creature you control, then roll a d20.
//!  1—14 | Create a tapped and attacking token that's a copy of that creature,
//!  except it's not legendary and it has 'At end of combat, exile this token.'
//!  15—20 | Create one of those tokens. You may roll again."
//!
//! The attack trigger and its "target creature you control" are wired. The d20
//! roll, the legendary-stripped tapped-and-attacking token copy with its own
//! end-of-combat exile, and the "roll again" loop are not expressible with the
//! available primitives (no dice, and CopyPermanent cannot strip legendary,
//! enter tapped-and-attacking, or carry a granted ability) — resolver is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Delina, Wild Mage");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: delina_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn delina_attack(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: d20 roll + legendary-stripped tapped-and-attacking copy with a granted
    // "at end of combat, exile this token" ability + "roll again" loop are not
    // expressible with the available primitives.
    Vec::new()
}
