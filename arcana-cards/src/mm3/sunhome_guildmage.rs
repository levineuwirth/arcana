//! Sunhome Guildmage — `{R}{W}` 2/2 Human Wizard with two activated abilities.
//!
//! Oracle text:
//! * "{1}{R}{W}: Creatures you control get +1/+0 until end of turn." — a
//!   board-wide pump over every creature you control, expressed via
//!   `Effect::ForEach` over `script::ids_matching`.
//! * "{2}{R}{W}: Create a 1/1 red and white Soldier creature token with
//!   haste." — `Effect::CreateToken`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::effects::TokenDefinition;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunhome Guildmage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    // Pre-intern the token subtype at registration time so the resolver's
    // read-only `reg.interner().lookup("Soldier")` resolves.
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{W}: Creatures you control get +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_your_creatures,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}{W}: Create a 1/1 red and white Soldier creature token with haste.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_soldier_token,
            }),
    )
}

fn pump_your_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: arcana_core::objects::NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}

fn make_soldier_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: soldier,
            colors: ColorSet::red() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
}
