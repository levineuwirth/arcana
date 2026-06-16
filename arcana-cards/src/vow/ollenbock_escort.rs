//! Ollenbock Escort — `{W}` 1/1 Human Cleric with Vigilance.
//! "Sacrifice this creature: Target creature you control with a +1/+1
//! counter on it gains lifelink and indestructible until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ollenbock Escort");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice this creature: Target creature you control with a +1/+1 counter on it gains lifelink and indestructible until end of turn.".into(),
            cost: ActivationCost { sacrifice: true, ..ActivationCost::default() },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter {
                    has_counter: Some(CounterKind::PlusOnePlusOne),
                    ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
                }),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_lifelink_indestructible,
        }),
    )
}

fn grant_lifelink_indestructible(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
    ]
}
