//! Wayta, Trainer Prodigy — `{R}{G}{W}` 1/5 Legendary Human Warrior with Haste.
//!
//! * Haste (keyword).
//! * `{2}{G}, {T}: Target creature you control fights another target creature.`
//!   (The "costs {2} less if it targets two creatures you control" reduction is a
//!   conditional cost adjustment not expressible via ActivationCost — GAP'd.)
//! * Static damage-replication trigger ("an additional time") — not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wayta, Trainer Prodigy");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "costs {2} less if it targets two creatures you control" — conditional
            // cost reduction is not expressible via ActivationCost; flat {2}{G},{T} modeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}, {T}: Target creature you control fights another target creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: fight_two,
            }),
    )
    // GAP: static damage-trigger-replication ("triggers an additional time") — no Effect/
    // ability variant models replicating another permanent's triggered ability.
}

fn fight_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut ids = ctx.targets.targets.iter().filter_map(|t| match t {
        TargetChoice::Object(id) => Some(*id),
        _ => None,
    });
    let (Some(a), Some(b)) = (ids.next(), ids.next()) else {
        return Vec::new();
    };
    vec![Effect::Fight { a, b }]
}
