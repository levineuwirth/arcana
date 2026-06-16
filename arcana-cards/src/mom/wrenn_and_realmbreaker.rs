//! Wrenn and Realmbreaker — `{1}{G}{G}` Legendary Planeswalker — Wrenn,
//! starting loyalty 5.
//!
//! Lands you control have "{T}: Add one mana of any color."
//! +1: Up to one target land you control becomes a 3/3 Elemental creature with
//!     vigilance, hexproof, and haste until your next turn. It's still a land.
//! −2: Mill three cards. You may put a permanent card from among the milled
//!     cards into your hand.
//! −7: You get an emblem with "You may play lands and cast permanent spells from
//!     your graveyard."
//!
//! # Scope
//! - The static ("Lands you control have '{T}: Add one mana of any color.'") is
//!   a granted-mana-ability static, not a loyalty ability; GAP'd (no loyalty
//!   cost — it's a continuous ability of the planeswalker itself).
//! - The `+1` ANIMATES an optional target land you control into a 3/3 creature,
//!   still a land (AddType is additive), with vigilance/hexproof/haste until
//!   your next turn. The Elemental subtype isn't addable via the demonstrated
//!   surface (no AddSubtype) — P/T, creature type, and keywords ARE applied.
//! - The `−2` mills three; the "you may put a permanent card from among the
//!   milled cards into your hand" pick is GAP'd (no milled-subset pick in the
//!   demonstrated surface). The Mill IS applied.
//! - The `−7` emblem grant ("play lands / cast permanent spells from your
//!   graveyard") is a rule-altering permission the anthem/keyword/filtered
//!   builders can't express; emblem shell created, static GAP'd.
//!
//! GAP: Scryfall tagged "Mill" — the mill lives inside the `−2` loyalty
//! ability, so no card-level keyword is emitted.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn and Realmbreaker");
    let wrenn = reg.interner_mut().intern("Wrenn");
    let _emblem = reg.interner_mut().intern("Wrenn and Realmbreaker emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wrenn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target land you control becomes a 3/3 \
                       Elemental creature with vigilance, hexproof, and haste \
                       until your next turn. It's still a land."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_animate_land,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Mill three cards. You may put a permanent card from \
                       among the milled cards into your hand."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"You may play lands and cast \
                       permanent spells from your graveyard.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1: Target land you control becomes a 3/3 creature, still a land.`
fn plus_one_animate_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    let dur = Duration::UntilYourNextTurn(ctx.controller);
    vec![
        Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: dur,
        },
        Effect::SetBasePT {
            target: id,
            power: 3,
            toughness: 3,
            duration: dur,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Vigilance,
            duration: dur,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Hexproof,
            duration: dur,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: dur,
        },
    ]
}

/// `−2: Mill three cards. (Milled-card pick GAP'd.)`
fn minus_two_mill(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // The mill IS applied; the "you may put a permanent card from among the
    // milled cards into your hand" pick is GAP'd — no milled-subset pick in the
    // demonstrated surface.
    vec![Effect::Mill { player: ctx.controller, count: 3 }]
}

/// `−7: You get an emblem with "play lands / cast permanent spells from gy."`
fn minus_seven_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Wrenn and Realmbreaker emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: rule-altering permission ("play lands and cast permanent
            // spells from your graveyard") not expressible by anthem/keyword/
            // filtered builders. Emblem shell still created.
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
