//! Candlekeep Sage — `{2}{U}` Legendary Enchantment — Background.
//! "Commander creatures you own have 'When this creature enters or
//! leaves the battlefield, draw a card.'"
//!
//! Implementation: an ETB trigger installs a
//! `ContinuousEffect::filtered_grant_triggered` (Background recipe)
//! granting commander creatures a `SelfEntersBattlefield` →
//! draw-a-card triggered ability. "You own" is approximated by
//! `controlled_by(ControllerConstraint::You)`. The designation is
//! inert outside commander games — faithful reading, wired anyway.
//!
//! GAP: only the "enters" half is granted. "…or LEAVES the
//! battlefield" has no expressible TriggerCondition (no
//! SelfLeavesBattlefield; ZoneChange requires a concrete `to` zone,
//! and a granted SelfDies would not fire — the grant scan only
//! matches battlefield objects at event time). The granted ability
//! under-fires on leave, never over-fires.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Candlekeep Sage");
    let background = reg.interner_mut().intern("Background");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(background);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_grant(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "…or leaves the battlefield" half of the granted
    // trigger is not expressible (no leaves-battlefield condition
    // reachable from the grant scan); only the enters half is wired.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_grant_triggered(
            trig.source,
            ObjectFilter::creature()
                .commander_only()
                .controlled_by(ControllerConstraint::You),
            TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: granted_draw_on_enter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Granted ability: "When this creature enters the battlefield, draw
/// a card." (`trig.controller` is the commander creature's controller.)
fn granted_draw_on_enter(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
