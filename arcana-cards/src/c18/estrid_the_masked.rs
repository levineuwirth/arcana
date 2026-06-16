//! Estrid, the Masked — `{1}{G}{W}{U}` Legendary Planeswalker — Estrid, loyalty 5.
//!
//! +2: Untap each enchanted permanent you control.
//! −1: Create a white Aura enchantment token named Mask attached to another
//!   target permanent. The token has enchant permanent and umbra armor.
//! −7: Mill seven cards. Return all non-Aura enchantment cards from your
//!   graveyard to the battlefield, then do the same for Aura cards.
//!
//! # Scope
//! GAP: the +2 "untap each enchanted permanent you control" needs an
//!   "is-enchanted" object filter not in the demonstrated surface. Ability shell
//!   declared, effect empty.
//! GAP: the −1 Aura-token creation (a token with enchant-permanent + umbra
//!   armor, created already attached to a target) isn't expressible. Ability
//!   shell declared, effect empty.
//! GAP: the −7 "return ALL [non-Aura then Aura] enchantments from your
//!   graveyard" is mass reanimation; the demonstrated `Reanimate` returns a
//!   single chosen card. Only the "Mill seven cards" half is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Estrid, the Masked");
    let estrid = reg.interner_mut().intern("Estrid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(estrid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Untap each enchanted permanent you control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Create a white Aura enchantment token named Mask attached to another target permanent. The token has enchant permanent and umbra armor.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Mill seven cards. Return all non-Aura enchantment cards from your graveyard to the battlefield, then do the same for Aura cards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_mill,
            }),
    )
}

fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "untap each enchanted permanent you control" needs an is-enchanted
    // object filter not in the demonstrated surface.
    Vec::new()
}

fn minus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Aura-token creation (enchant permanent + umbra armor, created
    // already attached) isn't expressible.
    Vec::new()
}

fn minus_seven_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return ALL [non-Aura then Aura] enchantments" is mass reanimation;
    // the demonstrated Reanimate returns a single chosen card. Only the
    // "Mill seven cards" half is modeled.
    vec![Effect::Mill { player: ctx.controller, count: 7 }]
}
