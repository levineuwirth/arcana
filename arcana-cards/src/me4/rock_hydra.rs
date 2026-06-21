//! Rock Hydra — `{X}{R}{R}` 0/0 Creature — Hydra.
//!
//! Oracle:
//! * This creature enters with X +1/+1 counters on it.
//!   (GAP: X is the value chosen for the {X} in the cast cost; it is not
//!   available to a SelfEntersBattlefield resolver via the demonstrated
//!   `script::` helpers, so the enter-with-X counters are not emitted.)
//! * For each 1 damage that would be dealt to this creature, if it has a
//!   +1/+1 counter on it, remove a +1/+1 counter and prevent that 1 damage.
//!   (GAP: a static per-1-damage replacement that consumes a counter is not in
//!   the demonstrated surface.)
//! * {R}: Prevent the next 1 damage that would be dealt to this creature this
//!   turn. — modeled as a mana-cost activated ability that prevents 1 damage to
//!   the source until end of turn.
//! * {R}{R}{R}: Put a +1/+1 counter on this creature. Activate only during your
//!   upkeep. — modeled as a mana-cost activated ability adding a +1/+1 counter
//!   to the source. (GAP: the "only during your upkeep" timing restriction is
//!   not expressible with the demonstrated ActivationCost fields.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rock Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: Prevent the next 1 damage that would be dealt to this creature this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_one_damage_to_self,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}{R}: Put a +1/+1 counter on this creature. Activate only during your upkeep."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_one_counter_to_self,
            }),
    )
}

fn prevent_one_damage_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: Some(1),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn add_one_counter_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
