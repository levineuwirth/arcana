//! Rinoa Heartilly — `{3}{G}{W}` 4/4 Legendary Human Rebel Warlock (G/W).
//! "When Rinoa Heartilly enters, create Angelo, a legendary 1/1 green and
//! white Dog creature token."
//! "Angelo Cannon — Whenever Rinoa Heartilly attacks, another target creature
//! you control gets +1/+1 until end of turn for each creature you control."
//!
//! "Angelo Cannon" is the flavor name of the attack trigger, not a keyword.
//! The ETB creates the Angelo Dog token (TokenDefinition has no supertype
//! field, so the token's legendary status is a fidelity GAP). The attack
//! trigger pumps another target creature you control by N/N where N is the
//! number of creatures you control.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Rinoa Heartilly");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let warlock = reg.interner_mut().intern("Warlock");
    let _dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_angelo,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pump_other,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_create_angelo(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Angelo's legendary supertype — TokenDefinition has no supertype field.
    let dog = match reg.interner().lookup("Dog") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dog,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn attack_pump_other(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    vec![Effect::Pump {
        target: *id,
        power: n,
        toughness: n,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
