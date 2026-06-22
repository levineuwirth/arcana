//! Troyan, Gutsy Explorer — `{1}{G}{U}` 1/3 Legendary Vedalken Scout (green/blue).
//!
//! * "{T}: Add {G}{U}. Spend this mana only to cast spells with mana value 5
//!   or greater or spells with {X} in their mana costs." → a mana ability
//!   (tap to add {G}{U}). GAP (rider): the "spend this mana only to cast …"
//!   restriction (CR 106.7) is not modeled — the mana is produced without the
//!   spend restriction (same posture as Powerstone).
//! * "{U}, {T}: Draw a card, then discard a card." → an activated ability.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Troyan, Gutsy Explorer");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}{U}. Spend this mana only to cast spells \
                       with mana value 5 or greater or spells with {X} in \
                       their mana costs."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_gu,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Draw a card, then discard a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot,
            }),
    )
}

fn add_gu(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, ctx.source),
            ManaUnit::plain(ManaColor::Blue, ctx.source),
        ],
    }]
}

fn loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
