//! Obscura Initiate — `{2}{U}` 2/2 Bird Citizen with Flying.
//!
//! Oracle:
//! * Flying
//! * `{1}{W/B}: This creature gains lifelink until end of turn.`
//!
//! Flying is a base keyword; the activation grants lifelink to itself until
//! end of turn via `Effect::GrantKeyword`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Obscura Initiate");
    let bird = reg.interner_mut().intern("Bird");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{W/B}: This creature gains lifelink until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{W/B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_lifelink,
        }),
    )
}

fn gain_lifelink(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Lifelink,
        duration: Duration::EndOfTurn,
    }]
}
