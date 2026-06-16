//! General Jarkeld — `{3}{W}` Legendary 1/2 Human Soldier.
//! "{T}: Choose two target blocked attacking creatures. If each of those creatures
//!  could be blocked by all creatures that the other is blocked by, each creature
//!  that's blocking exactly one of those attacking creatures stops blocking it and
//!  is blocking the other attacking creature. Activate only during the declare
//!  blockers step."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("General Jarkeld");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: "Activate only during the declare blockers step" — a timing window
            //      with no expressible activation-condition field.
            text: "{T}: Choose two target blocked attacking creatures. ... reassign their blockers. Activate only during the declare blockers step.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature()),
                count: TargetCount::Exactly(2),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: reassign_blockers,
        }),
    )
}

fn reassign_blockers(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the blocker-swap effect (conditional reassignment of all creatures
    //      blocking exactly one of the two chosen attackers to the other) has no
    //      expressible primitive in the Effect catalog; the activated ability's
    //      bones (tap cost, two creature targets) are emitted but the body is omitted.
    Vec::new()
}
