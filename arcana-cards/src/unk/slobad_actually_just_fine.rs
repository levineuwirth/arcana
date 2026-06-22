//! Slobad, Actually Just Fine — `{1}{B}{R}` 3/3 Legendary Goblin Artificer.
//!
//! Oracle:
//! * "You can sacrifice creatures and artifacts interchangeably." — a
//!   static replacement on the sacrifice-cost-matching rules; not
//!   expressible with the demonstrated API. GAP'd below.
//! * `{1}, {T}, Sacrifice another artifact or creature: Put a rebuilding
//!   counter on Slobad.` — activated, sacrifice_other (artifact OR
//!   creature), add a named "rebuilding" counter on the source.
//! * `{T}, Remove 3 Rebuilding Counters from Slobad: Create a token
//!   that's a copy of Bosh, Iron Golem.` — activated; remove_self_counter
//!   pays the 3 counters; the token-copy of a specific NAMED card is not
//!   expressible (CopyPermanent copies an on-battlefield object, not a
//!   card-by-name blueprint). GAP the effect body.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slobad, Actually Just Fine");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    let rebuilding = reg.interner_mut().intern("rebuilding");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "You can sacrifice creatures and artifacts interchangeably"
    // (sacrifice-cost replacement) is not expressible with this API.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice another artifact or creature: Put a rebuilding counter on Slobad.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_rebuilding_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove 3 Rebuilding Counters from Slobad: Create a token that's a copy of Bosh, Iron Golem.".into(),
                cost: ActivationCost {
                    tap: true,
                    remove_self_counter: Some((CounterKind::Named(rebuilding), 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_bosh,
            }),
    )
}

fn put_rebuilding_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(rebuilding) = reg.interner().lookup("rebuilding") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Named(rebuilding),
        count: 1,
    }]
}

fn copy_bosh(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create a token that's a copy of Bosh, Iron Golem" — CopyPermanent
    // copies an on-battlefield object by id, not a NAMED card blueprint; no
    // create-token-copy-of-named-card primitive exists.
    Vec::new()
}
