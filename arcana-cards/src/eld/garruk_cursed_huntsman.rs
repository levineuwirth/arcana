//! Garruk, Cursed Huntsman — `{4}{B}{G}` legendary planeswalker, starting loyalty 5.
//! B/G planeswalker (subtype Garruk).
//!
//! # Rules references
//!
//! * CR 606 — loyalty abilities; CR 606.3 — sorcery-speed, controller-only,
//!   once per turn per planeswalker. CR 704.5i — 0-loyalty sacrifice SBA.
//!
//! # Scope
//!
//! * `0`: Create two 2/2 black-and-green Wolf creature tokens. Faithful for
//!   the token bodies; the tokens' printed death trigger ("put a loyalty
//!   counter on each Garruk you control") is a per-each board effect not
//!   expressible from the demonstrated surface, so the tokens are minted
//!   without that ability (documented GAP below).
//! * `−3`: Destroy target creature, then draw a card — fully expressed.
//! * `−6`: Emblem — GAP (emblem creation is not in the demonstrated surface).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Cursed Huntsman");
    let garruk = reg.interner_mut().intern("Garruk");
    // Intern the Wolf token's subtype/name so the resolver can look it up.
    let _wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create two 2/2 black and green Wolf creature tokens \
                       with \"When this token dies, put a loyalty counter on \
                       each Garruk you control.\"".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_make_wolves,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target creature. Draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"Creatures you control get \
                       +3/+3 and have trample.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `0: Create two 2/2 black-and-green Wolf creature tokens.`
fn zero_make_wolves(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: each token's "When this token dies, put a loyalty counter on
    // each Garruk you control" is a per-each board effect not expressible
    // from the demonstrated surface; tokens are minted without it.
    let wolf = reg.interner().lookup("Wolf").expect("Wolf interned at register");
    let mut wolf_subtypes = SubtypeSet::default();
    wolf_subtypes.0.insert(wolf);
    let token = TokenDefinition {
        name: reg.interner().lookup("Wolf").expect("Wolf interned at register"),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: wolf_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}

/// `−3: Destroy target creature. Draw a card.`
fn minus_three_destroy_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return vec![Effect::DrawCards { player: ctx.controller, count: 1 }];
    };
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}

/// `−6: You get an emblem with "Creatures you control get +3/+3 and have trample."`
fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not in the demonstrated effect surface.
    Vec::new()
}
