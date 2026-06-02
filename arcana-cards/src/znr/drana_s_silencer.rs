//! Drana's Silencer — `{5}{B}` 3/2 black Vampire Rogue. "When this
//! creature enters, target creature an opponent controls gets -X/-X
//! until end of turn, where X is the number of creatures in your
//! party. (Your party consists of up to one each of Cleric, Rogue,
//! Warrior, and Wizard.)"

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Drana's Silencer");
    let vampire = reg.interner_mut().intern("Vampire");
    let rogue = reg.interner_mut().intern("Rogue");
    // Pre-intern the party subtypes so the resolver can look them up.
    let _cleric = reg.interner_mut().intern("Cleric");
    let _rogue2 = reg.interner_mut().intern("Rogue");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_minus_x_party,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

/// X = the number of creatures in your party (up to one each of
/// Cleric, Rogue, Warrior, Wizard). Computed as the number of those
/// four subtypes present among creatures you control, capped at 4.
fn etb_minus_x_party(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut party: i32 = 0;
    for st in ["Cleric", "Rogue", "Warrior", "Wizard"] {
        let filter = script::subtype_filter(reg, st)
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, trig.controller) > 0 {
            party += 1;
        }
    }
    vec![Effect::Pump {
        target: *id,
        power: -party,
        toughness: -party,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
