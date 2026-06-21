//! Tyvar, the Pummeler — `{1}{G}{G}` 3/3 Legendary Elf Warrior.
//! "Tap another untapped creature you control: Tyvar gains
//! indestructible until end of turn. Tap it."
//! "{3}{G}{G}: Creatures you control get +X/+X until end of turn, where
//! X is the greatest power among creatures you control."
//!
//! Ability 1 is wired: the cost taps another untapped creature you
//! control (tap_other), the effect grants Tyvar indestructible. Ability
//! 2's "+X/+X where X is the greatest power among creatures you control"
//! has no max-power script helper (script:: offers no "greatest power"
//! reduction), so its body is GAP'd to avoid a wrong literal.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyvar, the Pummeler");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap another untapped creature you control: Tyvar gains indestructible until end of turn."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_indestructible,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}{G}: Creatures you control get +X/+X until end of turn, where X is the greatest power among creatures you control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_team_by_max_power,
            }),
    )
}

fn gain_indestructible(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Indestructible,
        duration: Duration::EndOfTurn,
    }]
}

fn pump_team_by_max_power(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+X/+X ... where X is the greatest power among creatures you control"
    // — there is no script:: helper that reduces to the maximum power across a
    // board, so the dynamic X cannot be computed; emitting a literal would be
    // materially wrong.
    Vec::new()
}
