//! Dihada, Binder of Wills — `{1}{R}{W}{B}` Legendary Planeswalker — Dihada,
//! starting loyalty 5.
//!
//! Oracle:
//! * `+2`: Up to one target legendary creature gains vigilance, lifelink,
//!   and indestructible until your next turn.
//! * `−3`: Reveal the top four cards of your library. Put any number of
//!   legendary cards from among them into your hand and the rest into your
//!   graveyard. Create a Treasure token for each card put into your
//!   graveyard this way.
//! * `−11`: Gain control of all nonland permanents until end of turn. Untap
//!   them. They gain haste until end of turn.
//!
//! "Dihada, Binder of Wills can be your commander." — a commander-eligibility
//! line, not a loyalty ability; ignored.
//!
//! # Rules references
//! * CR 606 — loyalty abilities; engine enforces sorcery-speed / stack-empty
//!   / controller-only / once-per-turn / 0-loyalty SBA.
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//!
//! # Scope
//! * `+2` IMPLEMENTED: up-to-one target legendary creature gains three
//!   keywords until the controller's next turn (`Duration::UntilYourNextTurn`).
//! * `−11` IMPLEMENTED: sweeps all nonland permanents and grants each a
//!   `ChangeControlEot` + `Untap` + haste. This faithfully grabs ALL nonland
//!   permanents (including Dihada and the controller's own), matching oracle.
//! * `−3` GAP'd: reveal-top-4-then-sort-by-pick into hand/graveyard with a
//!   dynamic Treasure count (one per card binned to graveyard) has no
//!   reveal-and-sort primitive, and the Treasure count is runtime-dependent
//!   (`Effect::CreateCommodityToken` exists but the COUNT is unknowable here).
//!   The `−3` cost is declared correctly; the effect body is empty.
//!
//! The Scryfall "Treasure" keyword is not a card keyword — it surfaces only in
//! the `−3` text (GAP'd), so no `keywords` are emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dihada, Binder of Wills");
    let dihada = reg.interner_mut().intern("Dihada");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dihada);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Up to one target legendary creature gains \
                       vigilance, lifelink, and indestructible until your \
                       next turn."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_supertypes(
                            SupertypeSet(SupertypeSet::LEGENDARY),
                        ),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_grant,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Reveal the top four cards of your library. Put any \
                       number of legendary cards from among them into your \
                       hand and the rest into your graveyard. Create a \
                       Treasure token for each card put into your graveyard \
                       this way."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_reveal_sort,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−11: Gain control of all nonland permanents until end \
                       of turn. Untap them. They gain haste until end of turn."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 11)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eleven_mass_control,
            }),
    )
}

/// `+2: Up to one target legendary creature gains vigilance, lifelink, and
/// indestructible until your next turn.`
fn plus_two_grant(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Up to one" — no target chosen is legal; return nothing.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    let dur = Duration::UntilYourNextTurn(ctx.controller);
    vec![
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Vigilance,
            duration: dur,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Lifelink,
            duration: dur,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Indestructible,
            duration: dur,
        },
    ]
}

/// `−3: Reveal the top four cards of your library. Put any number of legendary
/// cards from among them into your hand and the rest into your graveyard.
/// Create a Treasure token for each card put into your graveyard this way.`
fn minus_three_reveal_sort(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no reveal-top-N-and-sort-by-pick primitive (cards split into hand
    // vs graveyard by a runtime "any number" choice), and the Treasure count
    // is the number binned to graveyard — unknowable at build time, so
    // Effect::CreateCommodityToken can't be emitted with a fixed count.
    Vec::new()
}

/// `−11: Gain control of all nonland permanents until end of turn. Untap them.
/// They gain haste until end of turn.`
fn minus_eleven_mass_control(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
        ctx.controller,
    );
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::ChangeControlEot {
            target: id,
            new_controller: ctx.controller,
        });
        effects.push(Effect::Untap { target: id });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
