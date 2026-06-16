//! Teyo, the Shieldmage — `{2}{W}` Legendary Planeswalker — Teyo.
//! Printed starting loyalty 5 (CR 113.3c). Color white.
//!
//! Static: "You have hexproof." — a continuous static granting the
//! controller hexproof; not a loyalty ability and not expressible as a
//! player-targeting static from this card class. GAP.
//!
//! Loyalty abilities (CR 606):
//! * `−2`: Create a 0/3 white Wall creature token with defender. —
//!   expressible.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teyo, the Shieldmage");
    let teyo = reg.interner_mut().intern("Teyo");
    let _wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };
    // GAP: "You have hexproof" controller-static omitted.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "−2: Create a 0/3 white Wall creature token with defender.".into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Loyalty, 2)),
                ..ActivationCost::default()
            },
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: true,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: minus_two_wall,
        }),
    )
}

/// `−2`: create the 0/3 Wall.
fn minus_two_wall(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wall = reg
        .interner()
        .lookup("Wall")
        .expect("Wall interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);
    let token = TokenDefinition {
        name: wall,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}
