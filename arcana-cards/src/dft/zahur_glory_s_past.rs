//! Zahur, Glory's Past — `{W}{B}` 3/2 Legendary Zombie Cat Warrior.
//!
//! "Start your engines! (speed mechanic)
//!  Sacrifice another creature: Surveil 1. Activate only once each turn.
//!  Max speed — Whenever a nontoken creature you control dies, create a tapped
//!  2/2 black Zombie creature token."
//!
//! The Scryfall keyword tags (Surveil / Max speed / Start your engines!) are
//! not `KeywordAbility` variants → keywords empty. "Start your engines!" and the
//! speed mechanic are unmodeled (no speed primitive) — GAP'd.
//!
//! The activated ability (sacrifice another creature → Surveil 1, once per
//! turn) IS expressible. The death trigger fires when a nontoken creature you
//! control dies and creates a 2/2 black Zombie token.

// GAP (mechanic): "Start your engines!" and speed are not modeled — no speed
// primitive, and the "Max speed —" gate on the death trigger cannot be
// expressed, so that trigger fires regardless of speed.
// GAP (fidelity): the created Zombie token enters UNTAPPED — there is no
// create-a-tapped-(non-attacking)-token Effect.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zahur, Glory's Past");
    let zombie = reg.interner_mut().intern("Zombie");
    let cat = reg.interner_mut().intern("Cat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another creature: Surveil 1. Activate only once each turn."
                    .into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: surveil_one,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "Max speed —" gate (speed must be 4) is not modeled; the
                // trigger fires regardless of speed.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: make_zombie,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn surveil_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: ctx.controller,
        count: 1,
    }]
}

fn make_zombie(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
