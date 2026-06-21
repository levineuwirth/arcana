//! Pep, Raucous Raider — `{2}{R}{G}` 3/3 Legendary Creature — Noggle Rogue.
//!
//! Oracle:
//! * Trample, haste.
//! * Whenever a creature you control deals combat damage to a player, exile the
//!   top card of that player's library. It perpetually becomes an artifact if
//!   it's a nonland permanent card. Until end of turn, you may play the exiled
//!   card. (GAP: no exile-from-library-top, no perpetual, no play-from-exile of
//!   another player's card; trigger fires but body is GAP'd.)
//! * {T}, Sacrifice an artifact: Add three mana of any one color. (GAP: no
//!   any-one-color AddMana primitive; the cost shape is modeled.)
//!
//! Decomposition: Trample + Haste → `keywords`; the combat-damage trigger →
//! one `TriggeredAbilityDef`; the tap/sac-artifact mana ability → one
//! `ActivatedAbilityDef`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pep, Raucous Raider");
    let noggle = reg.interner_mut().intern("Noggle");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(noggle);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: exile_top_perpetual,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice an artifact: Add three mana of any one color.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
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
                effect: add_any_one_color,
            }),
    )
}

fn exile_top_perpetual(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of that player's library; it perpetually becomes
    // an artifact; until end of turn you may play it" — no exile-from-library-
    // top, no perpetual modification, no play-from-exile of another's card.
    Vec::new()
}

fn add_any_one_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add three mana of any one color" — no player-chosen-color AddMana
    // primitive.
    Vec::new()
}
