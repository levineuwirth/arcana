//! Brightling — `{1}{W}{W}` 3/3 Shapeshifter.
//! `{W}: This creature gains vigilance until end of turn.`
//! `{W}: This creature gains lifelink until end of turn.`
//! `{W}: Return this creature to its owner's hand.`
//! `{1}: This creature gets +1/-1 or -1/+1 until end of turn.`

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
    let name = reg.interner_mut().intern("Brightling");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: This creature gains vigilance until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_vigilance,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: This creature gains lifelink until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_lifelink,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: Return this creature to its owner's hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bounce_self,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: This creature gets +1/-1 or -1/+1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: shift_pt,
            }),
    )
}

fn gain_vigilance(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Vigilance,
        duration: Duration::EndOfTurn,
    }]
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

fn bounce_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnToHand { target: ctx.source }]
}

fn shift_pt(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/-1 OR -1/+1" is a resolution-time modal choice between two pumps;
    // activated abilities have no modal dispatch, and there is no single Effect
    // that offers a choice between two Pump shapes, so this ability is omitted.
    Vec::new()
}
