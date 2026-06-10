//! Nantuko Monastery — nonbasic land.
//! "{T}: Add {C}." and "Threshold — {G}{W}: This land becomes a 4/4 green
//! and white Insect Monk creature with first strike until end of turn.
//! It's still a land. Activate only if there are seven or more cards in
//! your graveyard." The animation stacks AddType (keeps the land type) +
//! SetBasePT + SetColor + first strike; the Insect Monk subtypes and the
//! threshold activation gate are documented gaps.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nantuko Monastery");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Threshold — {G}{W}: This land becomes a 4/4 green \
                       and white Insect Monk creature with first strike \
                       until end of turn. It's still a land. Activate \
                       only if there are seven or more cards in your \
                       graveyard."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{W}")
                        .expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Activate only if there are seven or more cards in your
    // graveyard' (threshold) — no activation-condition gate is
    // expressible; the ability is wired unconditionally.
    // GAP: 'Insect Monk' — no effect adds creature SUBTYPES.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: ctx.source,
            colors: ColorSet::green() | ColorSet::white(),
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        },
    ]
}
