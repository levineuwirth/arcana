//! Longtusk Cub — `{1}{G}` 2/2 Cat.
//! "Whenever this creature deals combat damage to a player, you get {E}{E}."
//! "Pay {E}{E}: Put a +1/+1 counter on this creature."
//!
//! Decomposition:
//! * Combat-damage energy trigger — `DamageDealt` to a player (combat only) →
//!   gain two energy. (source_filter approximated as creatures you control, per
//!   the catalog idiom for "this creature deals combat damage" — no self-source
//!   DamageDealt variant exists.)
//! * Pay {E}{E} → +1/+1 counter activation — the +1/+1-counter payoff is wired,
//!   but paying energy as a cost is not a cost field; GAP'd cost side, so the
//!   ability is emitted with no cost (a documented fidelity gap).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Longtusk Cub");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "Pay {E}{E}" cost — energy is not a cost field. Cost side
            //      omitted; the +1/+1-counter payoff is emitted.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Pay {E}{E}: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter_self,
            }),
    )
}

fn combat_damage_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: 2,
    }]
}

fn add_counter_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
