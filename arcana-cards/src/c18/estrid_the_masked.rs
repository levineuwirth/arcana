//! Estrid, the Masked — `{1}{G}{W}{U}` Legendary Planeswalker — Estrid,
//! starting loyalty 5. Colors G/U/W.
//!
//! Oracle text:
//! * `+2`: Untap each enchanted permanent you control.
//! * `−1`: Create a white Aura enchantment token named Mask attached to
//!   another target permanent. The token has enchant permanent and umbra
//!   armor.
//! * `−7`: Mill seven cards. Return all non-Aura enchantment cards from
//!   your graveyard to the battlefield, then do the same for Aura cards.
//! * "Estrid, the Masked can be your commander." — a commander-eligibility
//!   static (CR 903), NOT a loyalty ability; nothing to model.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty
//!   (`loyalty: Some(5)`; `after_enter_battlefield` places them).
//! * CR 606 — loyalty abilities; CR 606.3 — sorcery-speed, stack empty,
//!   controller-only, once per turn per planeswalker (engine-enforced).
//! * CR 704.5i — 0-loyalty state-based sacrifice (engine-enforced).
//!
//! # Scope
//!
//! * Scryfall lists the keyword "Mill", but that is NOT a card keyword —
//!   it is the `−7` ability's effect. No `KeywordAbility` is emitted;
//!   `keywords: vec![]`.
//! * `+2` (untap each ENCHANTED permanent you control): GAP'd. An
//!   "enchanted permanent" = a permanent with an Aura/Equipment attached
//!   to it. `ObjectFilter` exposes no `enchanted` / `is_enchanted` /
//!   `attached` predicate (confirmed by reading `targets.rs`), so the set
//!   of enchanted permanents is not expressible from the demonstrated
//!   filter surface. The `+2` loyalty cost is still declared.
//! * `−1` (create a Mask Aura token already attached, with enchant
//!   permanent + umbra armor): GAP'd. The demonstrated `CreateToken` /
//!   `TokenDefinition` surface has no aura-attach-on-create and no
//!   umbra-armor primitive. The `−1` cost AND the correct "another target
//!   permanent" target requirement are declared.
//! * `−7` is IMPLEMENTED: `Effect::Mill { count: 7 }` then a scripted
//!   mass return — `Effect::Reanimate` resolves via a single PickCards
//!   choice (one card, not all), so it is NOT faithful for "return ALL".
//!   Instead we iterate the controller's graveyard, gather the ids of
//!   every non-Aura enchantment card and push one
//!   `Effect::ReturnFromGraveyardToBattlefield` per id, then do the same
//!   for Aura enchantment cards (Aura is a subtype — interned and used via
//!   `with_subtype_sym` / `without_subtype_sym`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

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
        // Printed starting loyalty; after_enter_battlefield places the
        // counters (CR 113.3c).
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
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_untap_enchanted,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Create a white Aura enchantment token named Mask \
                       attached to another target permanent. The token has \
                       enchant permanent and umbra armor."
                    .into(),
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
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_mask_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Mill seven cards. Return all non-Aura enchantment \
                       cards from your graveyard to the battlefield, then do \
                       the same for Aura cards."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_mill_reanimate,
            }),
    )
}

/// `+2: Untap each enchanted permanent you control.`
fn plus_two_untap_enchanted(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "enchanted permanent" (a permanent with an Aura/Equipment
    // attached) is not expressible — ObjectFilter has no
    // enchanted/is_enchanted/attached predicate in the demonstrated
    // surface, so the set of enchanted permanents can't be gathered.
    Vec::new()
}

/// `−1: Create a white Aura enchantment token named Mask attached to
/// another target permanent. The token has enchant permanent and umbra
/// armor.`
fn minus_one_mask_token(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: creating an Aura token ALREADY attached to a chosen permanent,
    // carrying enchant-permanent and umbra-armor (a protection-style
    // static), is not expressible by the demonstrated CreateToken /
    // TokenDefinition surface (no aura-attach-on-create, no umbra-armor
    // primitive). The −1 cost and the "another target permanent"
    // requirement are still declared.
    Vec::new()
}

/// `−7: Mill seven cards. Return all non-Aura enchantment cards from your
/// graveyard to the battlefield, then do the same for Aura cards.`
fn minus_seven_mill_reanimate(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = vec![Effect::Mill {
        player: ctx.controller,
        count: 7,
    }];

    // "Aura" is a subtype. Resolve its already-interned symbol from the
    // registry (catalog interns every card's subtypes at load); if it was
    // never interned, no card carries it, so the Aura set is empty and the
    // non-Aura set is simply "all enchantment cards".
    let enchantment = ObjectFilter::permanent().with_types(TypeLine::ENCHANTMENT.into());
    let (non_aura, aura_filter) = match reg.interner().lookup("Aura") {
        Some(sym) => (
            enchantment.clone().without_subtype_sym(sym),
            Some(enchantment.with_subtype_sym(sym)),
        ),
        None => (enchantment, None),
    };

    // Non-Aura enchantment cards first.
    let non_aura_ids: Vec<_> = state
        .objects
        .objects_in_zone(Zone::Graveyard(ctx.controller))
        .filter(|o| non_aura.matches(o, state, ctx.controller))
        .map(|o| o.id)
        .collect();
    for id in non_aura_ids {
        effects.push(Effect::ReturnFromGraveyardToBattlefield { target: id });
    }

    // Then Aura cards.
    if let Some(aura_filter) = aura_filter {
        let aura_ids: Vec<_> = state
            .objects
            .objects_in_zone(Zone::Graveyard(ctx.controller))
            .filter(|o| aura_filter.matches(o, state, ctx.controller))
            .map(|o| o.id)
            .collect();
        for id in aura_ids {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: id });
        }
    }

    effects
}
