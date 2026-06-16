//! Tyvar, Jubilant Brawler — `{1}{B}{G}` Legendary Planeswalker — Tyvar,
//! starting loyalty 3.
//!
//! Static (NOT a loyalty ability, GAP'd as a card-level static):
//! * "You may activate abilities of creatures you control as though those
//!   creatures had haste." — not expressible from the demonstrated surface.
//!
//! Loyalty abilities:
//! * `+1`: Untap up to one target creature.
//! * `−2`: Mill three cards, then you may return a creature card with mana
//!   value 2 or less from your graveyard to the battlefield. Mill 3 +
//!   `Reanimate` (filter: creature, mv ≤ 2, from graveyard).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyvar, Jubilant Brawler");
    let tyvar = reg.interner_mut().intern("Tyvar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyvar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap up to one target creature.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Mill three cards, then you may return a creature \
                       card with mana value 2 or less from your graveyard to \
                       the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            }),
    )
}

/// `+1: Untap up to one target creature.`
fn plus_one_untap(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Untap { target: *id }),
            _ => None,
        })
        .collect()
}

/// `−2: Mill three, then reanimate a creature mv ≤ 2 from your graveyard.`
fn minus_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Mill { player: ctx.controller, count: 3 },
        Effect::Reanimate {
            player: ctx.controller,
            filter: ObjectFilter::creature().with_max_cmc(2),
            from_zone: Zone::Graveyard(ctx.controller),
        },
    ]
}
