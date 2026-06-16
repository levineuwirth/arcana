//! Stiltzkin, Moogle Merchant — `{W}` 1/2 Legendary Moogle with Lifelink.
//! `{2}, {T}`: Target opponent gains control of another target permanent
//! you control. If they do, you draw a card.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Stiltzkin, Moogle Merchant");
    let moogle = reg.interner_mut().intern("Moogle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moogle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![arcana_core::effects::KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Target opponent gains control of another target permanent \
                       you control. If they do, you draw a card."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: donate_and_draw,
            }),
    )
}

fn donate_and_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut player = None;
    let mut perm = None;
    for t in &ctx.targets.targets {
        match t {
            TargetChoice::Player(p) => player = Some(*p),
            TargetChoice::Object(id) => perm = Some(*id),
            _ => {}
        }
    }
    let (Some(player), Some(perm)) = (player, perm) else { return Vec::new(); };
    vec![
        Effect::ChangeControl { target: perm, new_controller: player },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}
