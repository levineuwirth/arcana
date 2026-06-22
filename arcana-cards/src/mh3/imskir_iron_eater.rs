//! Imskir Iron-Eater — `{6}{B}{R}` 5/5 Legendary Demon.
//! Affinity for artifacts (keyword — GAP, cost reduction not expressible).
//! ETB: draw X / lose X life, X = half the number of artifacts you control.
//! `{3}{R}, Sacrifice an artifact: Imskir deals damage equal to the sacrificed
//! artifact's mana value to any target.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imskir Iron-Eater");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    // GAP: Affinity for artifacts is a cost-reduction keyword not in the usable
    // keyword surface; emit no keyword variant for it.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
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
                effect: etb_draw_lose,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}, Sacrifice an artifact: Imskir deals damage equal to the sacrificed artifact's mana value to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sac_for_damage,
            }),
    )
}

fn etb_draw_lose(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let artifacts = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let x = artifacts / 2;
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: x,
        },
        Effect::LoseLife {
            player: trig.controller,
            amount: x,
        },
    ]
}

fn sac_for_damage(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: damage amount equals the SACRIFICED artifact's mana value; the
    // already-paid sacrifice cost object's mana value isn't readable from the
    // activation context, so the dynamic amount can't be computed here. The
    // {3}{R} + Sacrifice-an-artifact cost and any-target are wired; the amount
    // is the only unexpressible piece.
    Vec::new()
}
