//! Koma, Cosmos Serpent — `{3}{G}{G}{U}{U}` 6/6 Legendary Serpent (G/U).
//! This spell can't be countered. (static — GAP)
//! At the beginning of each upkeep, create a 3/3 blue Serpent creature
//!   token named Koma's Coil.
//! Sacrifice another Serpent: Choose one —
//!   • Tap target permanent. Its activated abilities can't be activated
//!     this turn.
//!   • Koma gains indestructible until end of turn.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koma, Cosmos Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let _coil = reg.interner_mut().intern("Koma's Coil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    // GAP: "This spell can't be countered" is a cast-time static not
    // expressible with the demonstrated primitives.
    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of each upkeep, create a 3/3 blue Serpent token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: make_coil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Sacrifice another Serpent: Koma gains indestructible until end of turn.
            // GAP: modal "Choose one" cannot be modeled on an activated
            // ability — implementing the second mode (indestructible);
            // mode 1 (tap a permanent + abilities-can't-activate rider) is omitted.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another Serpent: Koma gains indestructible until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(script::subtype_filter(reg, "Serpent")),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_indestructible,
            }),
    )
}

fn make_coil(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let coil = reg.interner().lookup("Koma's Coil").unwrap_or_default();
    let serpent = reg.interner().lookup("Serpent").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    let token = TokenDefinition {
        name: coil,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn gain_indestructible(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Indestructible,
        duration: Duration::EndOfTurn,
    }]
}
