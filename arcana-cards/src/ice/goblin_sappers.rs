//! Goblin Sappers — `{1}{R}` 1/1 Goblin with two activated abilities.
//! "{R}{R}, {T}: Target creature you control can't be blocked this turn.
//!  Destroy it and this creature at end of combat."
//! "{R}{R}{R}{R}, {T}: Target creature you control can't be blocked this
//!  turn. Destroy it at end of combat."
//!
//! The "can't be blocked this turn" clause is expressible via
//! Effect::CantBeBlocked. The "destroy it [the target] / this creature at
//! end of combat" delayed-destruction riders are NOT expressible:
//! DelayedAction operates only on the scheduled source object (not an
//! arbitrary target) and DelayedWhen has no "end of combat" timing — so
//! those riders are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Sappers");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}, {T}: Target creature you control can't be blocked this turn. Destroy it and this creature at end of combat.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_unblockable,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}{R}{R}, {T}: Target creature you control can't be blocked this turn. Destroy it at end of combat.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}{R}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_unblockable,
            }),
    )
}

fn make_unblockable(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "Destroy it [and this creature] at end of combat" — a delayed
    // destruction of the chosen target (and source) at end-of-combat;
    // DelayedAction acts only on the scheduled source and DelayedWhen has
    // no end-of-combat timing, so only the unblockable clause is emitted.
    vec![Effect::CantBeBlocked {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}
