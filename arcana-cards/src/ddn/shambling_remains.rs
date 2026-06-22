//! Shambling Remains — `{1}{B}{R}` 4/3 Zombie Horror.
//! This creature can't block. (static — GAP'd)
//! Unearth {B}{R} ({B}{R}: Return this card from your graveyard to the
//! battlefield. It gains haste. Exile it at the beginning of the next end
//! step or if it would leave the battlefield. Unearth only as a sorcery.)

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shambling Remains");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);

    // GAP: static "This creature can't block" — a pure continuous static, not a
    // triggered/activated ability and no static-emitting primitive in this shape.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Unearth {B}{R} ({B}{R}: Return this card from your graveyard to the battlefield. It gains haste. Exile it at the beginning of the next end step or if it would leave the battlefield. Unearth only as a sorcery.)".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: unearth,
        }),
    )
}

fn unearth(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Return from graveyard, grant haste, and schedule an exile at the next end
    // step. (The "or if it would leave the battlefield" exile rider is not
    // separately expressible; the end-step exile is the dominant case.)
    vec![Effect::Sequence(vec![
        Effect::ReturnFromGraveyardToBattlefield { target: ctx.source },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Haste,
            duration: arcana_core::layers::Duration::WhileSourceOnBattlefield,
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Exile,
        },
    ])]
}
