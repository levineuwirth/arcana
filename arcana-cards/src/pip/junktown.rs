//! Junktown — nonbasic land (Fallout, 2024).
//! "{T}: Add {C}." and "{4}{R}, {T}, Sacrifice this land: Create three
//! Junk tokens." Junk tokens are artifacts with "{T}, Sacrifice this
//! token: Exile the top card of your library. You may play that card this
//! turn. Activate only as a sorcery." — Junk is not a wired commodity
//! token, so the tokens are minted as bare artifact tokens.
//! GAP: the Junk token's printed impulse activated ability is not wired
//! (TokenDefinition.abilities cannot carry it).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Junktown");
    let _junk = reg.interner_mut().intern("Junk");
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
                text: "{4}{R}, {T}, Sacrifice this land: Create three Junk \
                       tokens."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_junk,
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

fn make_junk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Junk token's printed "{T}, Sacrifice this token: Exile the
    // top card of your library. You may play that card this turn.
    // Activate only as a sorcery." activated ability is not wired.
    let junk = reg.interner().lookup("Junk").unwrap_or_default();
    (0..3)
        .map(|_| {
            let mut subtypes = SubtypeSet::default();
            subtypes.0.insert(junk);
            Effect::CreateToken {
                controller: ctx.controller,
                token: TokenDefinition {
                    name: junk,
                    colors: ColorSet::new(),
                    types: TypeLine::ARTIFACT.into(),
                    subtypes,
                    power: None,
                    toughness: None,
                    keywords: vec![],
                    abilities: vec![],
                },
            }
        })
        .collect()
}
