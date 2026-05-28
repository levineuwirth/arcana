//! Wakeroot Elemental — `{4}{G}{G}` 5/5 green Elemental.
//! "{G}{G}{G}{G}{G}: Untap target land you control. It becomes a 5/5 Elemental creature with
//! haste. It's still a land."
//!
//! GAP: "It becomes a 5/5 Elemental creature with haste. It's still a land." —
//! SetBasePT can set P/T but does not add creature type or haste while keeping land type.
//! Using Untap + SetBasePT + GrantKeyword as approximation; land-becomes-creature semantics are incomplete.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wakeroot Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}{G}{G}{G}{G}: Untap target land you control. It becomes a 5/5 Elemental creature with haste.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{G}{G}{G}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate_land,
            }),
    )
}

fn animate_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "becomes a creature while still a land" type-change semantics not fully expressible.
    vec![
        Effect::Untap { target: *id },
        Effect::SetBasePT { target: *id, power: 5, toughness: 5, duration: Duration::EndOfTurn },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Haste, duration: Duration::EndOfTurn },
    ]
}
