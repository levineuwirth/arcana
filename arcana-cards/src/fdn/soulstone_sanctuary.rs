//! Soulstone Sanctuary — nonbasic land.
//! "{T}: Add {C}." and "{4}: This land becomes a 3/3 creature with
//! vigilance and all creature types. It's still a land."
//!
//! The animation is modeled additively: `Effect::AddType` (creature —
//! additive, so it's still a land), `Effect::SetBasePT` 3/3, and
//! `Effect::GrantKeyword` for Vigilance plus Changeling ("all creature
//! types", CR 702.73).
//! GAP: the printed animation has no duration (it lasts indefinitely);
//! the closest expressible duration is
//! `Duration::WhileSourceOnBattlefield`.

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
    let name = reg.interner_mut().intern("Soulstone Sanctuary");
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
                text: "{4}: This land becomes a 3/3 creature with vigilance and all creature types. It's still a land.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
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
    // GAP: oracle animation is permanent (no duration); modeled with
    // Duration::WhileSourceOnBattlefield, the closest available duration.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 3,
            toughness: 3,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Changeling,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
