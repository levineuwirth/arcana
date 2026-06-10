//! Forger's Foundry — `{2}{U}` artifact (Murders at Karlov Manor /
//! Clue Edition). "{T}: Add {U}. When you spend this mana to cast an
//! instant or sorcery spell with mana value 3 or less, you may exile
//! that spell instead of putting it into its owner's graveyard as it
//! resolves." and "{3}{U}{U}, {T}: You may cast any number of spells
//! from among cards exiled with this artifact without paying their
//! mana costs. Activate only as a sorcery."
//! The plain {U} mana ability is wired; the spend-rider and the
//! cast-from-exile activation are honest GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::types::ManaColor;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forger's Foundry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}{U}, {T}: You may cast any number of spells \
                       from among cards exiled with this artifact without \
                       paying their mana costs. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}{U}")
                        .expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cast_from_exile,
            }),
    )
}

fn add_blue_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "When you spend this mana to cast an instant or sorcery spell
    // with mana value 3 or less, you may exile that spell instead of
    // putting it into its owner's graveyard as it resolves" — spend-rider
    // mana tracking and the resolve-to-exile replacement are not
    // expressible; the plain mana ability is emitted.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn cast_from_exile(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "cast any number of spells from among cards exiled with this
    // artifact without paying their mana costs" — no exiled-with-this
    // tracking and no free-cast-from-exile effect in the catalog.
    Vec::new()
}
