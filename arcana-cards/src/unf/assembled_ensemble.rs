//! Assembled Ensemble — `{4}{W}` */6 Artifact Creature — Clown Robot Bard.
//! Vigilance.
//! Assembled Ensemble's power is equal to the number of Robots you control.
//! (characteristic-defining ability — GAP)
//! Whenever you cast a spell with an artifact creature in its art, create a
//! 1/1 white Clown Robot artifact creature token. (effect — GAP)
//!
//! Vigilance is a base keyword. The "*" power is an ASYMMETRIC subtype CDA
//! (power = number of Robots you control, toughness fixed 6) — wired at Layer 7a
//! via `self_pt_from_match_asym` over a Robot-subtype filter built in the ETB fn,
//! which has the interner to name "Robot"; power printed as PtValue::Star.
//! The token-on-cast trigger fires on a `SpellCast` you cast (`caster: You`);
//! the "with an artifact creature in its art" restriction is an art-based
//! (Un-set) condition with no expressible filter, so the trigger CONDITION is
//! present (filter: None) but the token-creation EFFECT is GAP'd (firing on
//! every cast would over-create the token).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Assembled Ensemble");
    let clown = reg.interner_mut().intern("Clown");
    let robot = reg.interner_mut().intern("Robot");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clown);
    subtypes.0.insert(robot);
    subtypes.0.insert(bard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // "Whenever you cast a spell …" — the cast-by-you condition is
                // expressible; the "with an artifact creature in its art"
                // restriction is not, so we carry no filter and GAP the effect.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Whenever you cast a spell with an artifact creature in its art, create a
/// 1/1 white Clown Robot artifact creature token."
fn cast_gap(_s: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "an artifact creature in its art" is an art-based (Un-set) condition
    // with no expressible filter. Firing on every cast would over-create the
    // token, so the effect is GAP'd rather than approximated.
    Vec::new()
}

/// Power = Robots you control; toughness fixed 6.
fn install_cda(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let robots =
        script::subtype_filter(reg, "Robot").controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match_asym(
            trig.source,
            robots,
            /*count_is_power=*/ true,
            /*other_fixed=*/ 6,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
