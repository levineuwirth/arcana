//! Kaya, Intangible Slayer — `{3}{W}{W}{B}{B}` Legendary Planeswalker —
//! Kaya, starting loyalty 7. B/W. Has Hexproof.
//!
//! Oracle text:
//! * Hexproof
//! * `+2`: Each opponent loses 3 life and you gain 3 life.
//! * `0`: You draw two cards. Then each opponent may scry 1.
//! * `−3`: Exile target creature or enchantment. If it wasn't an Aura,
//!   create a token that's a copy of it, except it's a 1/1 white Spirit
//!   creature with flying in addition to its other types.
//!
//! # Scope
//!
//! * Hexproof is on the planeswalker itself (Characteristics keyword).
//! * `+2`: "you gain 3 life" is expressible; "each opponent loses 3
//!   life" requires a per-opponent life-loss sweep with no target —
//!   the dominant clause — so the whole ability is GAP'd to avoid an
//!   unfaithful partial.
//! * `0`: "you draw two cards" is expressed; the "each opponent may
//!   scry 1" rider (per-opponent optional scry) is not expressible and
//!   is omitted from the effect.
//! * `−3`: exile-then-conditional-copy-token is bespoke; GAP'd, cost
//!   shell declared.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya, Intangible Slayer");
    let kaya = reg.interner_mut().intern("Kaya");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}{B}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Hexproof],
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Each opponent loses 3 life and you gain 3 \
                       life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: You draw two cards. Then each opponent may \
                       scry 1.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Exile target creature or enchantment. If it \
                       wasn't an Aura, create a token that's a copy of it, \
                       except it's a 1/1 white Spirit creature with flying \
                       in addition to its other types.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            }),
    )
}

/// `+2`: each opponent loses 3, you gain 3.
fn plus_two(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each opponent loses 3 life" — per-opponent life-loss sweep
    // with no target — is not expressible; the dominant clause, so the
    // whole ability is GAP'd rather than emitting only the GainLife half.
    Vec::new()
}

/// `0`: draw two cards (the per-opponent optional scry is omitted).
fn zero(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each opponent may scry 1" (per-opponent optional scry) is
    // not expressible; the "you draw two cards" clause is faithful.
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}

/// `−3`: exile then conditional copy-token.
fn minus_three(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: exile-then-make-a-modified-copy-token is bespoke (copy with
    // an overridden P/T, color, type, and added flying) — not expressible.
    Vec::new()
}
