//! Nivix, Aerie of the Firemind — nonbasic land. "{T}: Add {C}." and
//! "{2}{U}{R}, {T}: Exile the top card of your library. Until your next
//! turn, you may cast it if it's an instant or sorcery spell."
//!
//! Modeled with `Effect::ImpulseExile` (exile top 1, play permission) —
//! fidelity gaps: the permission lasts until end of turn rather than
//! until your next turn, and the instant-or-sorcery restriction on the
//! cast is not enforced.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nivix, Aerie of the Firemind");
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
                text: "{2}{U}{R}, {T}: Exile the top card of your library. \
                       Until your next turn, you may cast it if it's an \
                       instant or sorcery spell."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{R}")
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
                effect: impulse_top_card,
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

fn impulse_top_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: permission window is until end of turn (engine ImpulseExile),
    // not "until your next turn"; the instant-or-sorcery cast
    // restriction is not enforced.
    vec![Effect::ImpulseExile { player: ctx.controller, count: 1 }]
}
