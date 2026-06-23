//! Dinosaurs on a Spaceship — `{4}{R}{W}` 7/7 Dinosaur.
//! Vigilance, trample.
//! "Other Dinosaurs you control get +1/+1 and have vigilance and trample."
//! Suspend 4—{3}{R}{W}. — GAP: Suspend is not in the implemented keyword
//! surface for this card class.
//! "Whenever a time counter is removed from this card while it's exiled, create
//! a 2/2 red and white Dinosaur creature token with flying and haste." — GAP:
//! there is no "counter removed" trigger condition, and the watched zone is
//! exile (suspend) rather than the battlefield.
//!
//! The anthem static is wired via a `SelfEntersBattlefield` trigger that
//! installs three continuous effects anchored to this creature with
//! `Duration::WhileSourceOnBattlefield`: a `filtered_pump` (+1/+1, layer 7c)
//! plus two `filtered_keyword` grants (Vigilance, Trample; layer 6) over
//! Dinosaurs you control. (The filter matches base characteristics, so the
//! source itself is included — a documented minor "OTHER" self-inclusion gap.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dinosaurs on a Spaceship");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        ..Default::default()
    };
    // GAP: Suspend 4—{3}{R}{W} — alternative-cast keyword not supported.
    // GAP: "time counter removed while exiled → make a Dinosaur token" — no
    //      counter-removal-while-exiled trigger variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_dinosaur_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "Dinosaurs you control get +1/+1 and have vigilance and
/// trample", anchored to this creature, lasting until it leaves play.
fn install_dinosaur_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dinosaur = reg
        .interner()
        .lookup("Dinosaur")
        .expect("Dinosaur interned during register()");
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(dinosaur);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                filter.clone(),
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter.clone(),
                KeywordAbility::Vigilance,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Trample,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
