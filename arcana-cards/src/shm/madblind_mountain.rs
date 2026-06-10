//! Madblind Mountain — nonbasic land, subtype Mountain.
//! "({T}: Add {R}.)", "This land enters tapped.", and "{R}, {T}:
//! Shuffle your library. Activate only if you control two or more
//! red permanents."
//!
//! GAP: "Shuffle your library" has no Effect variant, and the
//! "Activate only if you control two or more red permanents"
//! activation gate is not expressible on `ActivationCost` — the
//! utility activation is wired with its printed cost but resolves
//! to nothing.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Madblind Mountain");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mountain);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, {T}: Shuffle your library. Activate only if you \
                       control two or more red permanents."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: shuffle_library,
            }),
    )
}

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn shuffle_library(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Shuffle your library" — no shuffle-library Effect variant; and
    // "Activate only if you control two or more red permanents" — activation
    // precondition not expressible via ActivationCost.
    Vec::new()
}
