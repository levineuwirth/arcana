//! Witch-king of Angmar — `{3}{B}{B}` 5/3 Legendary Creature — Wraith Noble.
//!
//! Oracle:
//! * Flying
//! * Whenever one or more creatures deal combat damage to you, each opponent
//!   sacrifices a creature of their choice that dealt combat damage to you this
//!   turn. The Ring tempts you.  (effect GAP'd)
//! * Discard a card: Witch-king of Angmar gains indestructible until end of
//!   turn. Tap it.
//!
//! The combat-damage trigger fires correctly (`DamageDealt` from a creature
//! to a player, combat-only), but its payload is inexpressible: "sacrifices a
//! creature ... that dealt combat damage to you this turn" has no
//! sacrifice-filter for the attacker set, and "The Ring tempts you" has no
//! primitive. A plain "each opponent sacrifices a creature" would be a
//! materially different card, so the effect is GAP'd. The discard-activated
//! indestructible-and-tap ability is fully expressed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Witch-king of Angmar");
    let wraith = reg.interner_mut().intern("Wraith");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wraith);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: tempted_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a card: Witch-king of Angmar gains indestructible until end of turn. Tap it.".into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: indestructible_and_tap,
            }),
    )
}

fn tempted_sacrifice(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each opponent sacrifices a creature of their choice that dealt combat
    //      damage to you this turn" — no sacrifice-filter for the attacker set;
    //      "The Ring tempts you" has no primitive. A plain forced sacrifice would
    //      be a materially different card, so the whole effect is omitted.
    Vec::new()
}

fn indestructible_and_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::Tap { target: ctx.source },
    ]
}
