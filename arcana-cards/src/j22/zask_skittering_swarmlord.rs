//! Zask, Skittering Swarmlord — `{3}{G}{G}` 5/5 Legendary Creature — Insect.
//!
//! Oracle:
//! * You may play lands and cast Insect spells from your graveyard.
//!   (GAP — a static play-permission ability; no demonstrated API.)
//! * Whenever another Insect you control dies, put it on the bottom of its
//!   owner's library, then mill two cards. (Partial — the mill is wired; the
//!   "put the dying card on the bottom of its owner's library" from the
//!   graveyard has no demonstrated Effect variant and is GAP'd.)
//! * {1}{B/G}: Target Insect gets +1/+0 and gains deathtouch until end of
//!   turn.  (Activated, wired.)
//!
//! NOTE: Scryfall lists the reminder-mechanic "Mill" as a keyword; it is not a
//! `KeywordAbility` variant, so `keywords` stays empty.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zask, Skittering Swarmlord");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static "You may play lands and cast Insect spells from your
    // graveyard." — no demonstrated play-permission API.

    let insect_target = ObjectFilter::creature().with_subtype_sym(insect);

    reg.register(
        CardDefinition::new(name, chars)
            // Whenever another Insect you control dies, ... then mill two cards.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .with_subtype_sym(insect)
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_insect_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {1}{B/G}: Target Insect gets +1/+0 and gains deathtouch UEOT.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B/G}: Target Insect gets +1/+0 and gains deathtouch until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B/G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(insect_target),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_insect,
            }),
    )
}

fn on_insect_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put it on the bottom of its owner's library" — the dying creature is
    // already in the graveyard; there is no graveyard→bottom-of-library Effect.
    // The "then mill two cards" half is wired.
    vec![Effect::Mill { player: trig.controller, count: 2 }]
}

fn pump_insect(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Deathtouch],
    }]
}
