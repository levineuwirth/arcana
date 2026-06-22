//! The Motherlode, Excavator — `{3}{R}{R}` 5/5 Legendary Artifact Creature.
//! "When The Motherlode enters, choose target opponent. You get an amount of
//! {E} (energy counters) equal to the number of nonbasic lands that player
//! controls."
//! "Whenever The Motherlode attacks, you may pay {E}{E}{E}{E}. When you do,
//! destroy target nonbasic land defending player controls, and creatures that
//! player controls without flying can't block this turn."
//!
//! GAP (attack ability): the payload gates on paying {E}{E}{E}{E} — there is
//! no energy-payment cost shape (OptionalPayment only models Mana/Life), and
//! the reflexive "when you do" + can't-block clause hangs off that payment.
//! The attack trigger is emitted with a GAP'd effect. The ETB targeted-energy
//! ability IS fully expressed (dynamic count of the target opponent's nonbasic
//! lands).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Motherlode, Excavator");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_energy_from_nonbasics,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pay_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_energy_from_nonbasics(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(them) = target else {
        return Vec::new();
    };
    // Nonbasic lands the chosen player controls.
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    let n = script::count_matching(state, &filter, *them);
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: n,
    }]
}

fn attack_pay_energy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {E}{E}{E}{E}. When you do, destroy target nonbasic
    // land defending player controls, and creatures that player controls
    // without flying can't block this turn." No energy-payment cost shape.
    Vec::new()
}
