//! Inkmoth Nexus — nonbasic land.
//! "{T}: Add {C}." and "{1}: This land becomes a 1/1 Phyrexian
//! Blinkmoth artifact creature with flying and infect until end of
//! turn. It's still a land."
//! A colorless mana ability plus a self-animation activation:
//! additive AddType (artifact creature — the land type is kept, so
//! "it's still a land" holds), SetBasePT 1/1, and Flying + Infect
//! grants, all until end of turn.
//! GAP: the Phyrexian Blinkmoth creature subtypes cannot be added
//! (AddType covers card types only).

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
    let name = reg.interner_mut().intern("Inkmoth Nexus");
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
                text: "{1}: This land becomes a 1/1 Phyrexian Blinkmoth artifact creature with flying and infect until end of turn. It's still a land.".into(),
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
                effect: animate_self,
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

fn animate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Phyrexian Blinkmoth' subtypes not addable (AddType is card
    // types only).
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Infect,
            duration: Duration::EndOfTurn,
        },
    ]
}
