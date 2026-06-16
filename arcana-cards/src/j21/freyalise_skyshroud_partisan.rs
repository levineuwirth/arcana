//! Freyalise, Skyshroud Partisan — `{1}{G}{G}` legendary planeswalker, starting loyalty 3.
//!
//! +1: Untap up to one target Elf. That Elf and a random Elf card in
//!     your hand perpetually get +1/+1 (perpetual GAP).
//! −1: Seek an Elf card (Seek GAP).
//! −6: Conjure a card named Regal Force onto the battlefield (Conjure GAP).
//!
//! Scope: the +1 ability expresses the Untap of its target Elf; the
//! "perpetually get +1/+1" rider has no demonstrated Effect. Seek and
//! Conjure are Arena-only mechanics with no demonstrated surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Freyalise, Skyshroud Partisan");
    let freyalise = reg.interner_mut().intern("Freyalise");
    let _elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(freyalise);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let elf_sym = reg.interner().lookup("Elf").expect("interned");
    let elf_filter = arcana_core::targets::ObjectFilter::creature()
        .with_subtype_sym(elf_sym);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap up to one target Elf. That Elf and a random Elf \
                       creature card in your hand perpetually get +1/+1.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(elf_filter),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Seek an Elf card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_seek,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Conjure a card named Regal Force onto the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_conjure,
            }),
    )
}

/// `+1` — untap the target Elf (the perpetual +1/+1 rider is GAP'd).
fn plus_one_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "perpetually get +1/+1" rider (perpetual) not expressible.
    vec![Effect::Untap { target: *id }]
}

/// `−1` — Seek (Arena mechanic).
fn minus_one_seek(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Seek has no demonstrated Effect.
    Vec::new()
}

/// `−6` — Conjure (Arena mechanic).
fn minus_six_conjure(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Conjure has no demonstrated Effect.
    Vec::new()
}
