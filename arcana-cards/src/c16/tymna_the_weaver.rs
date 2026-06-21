//! Tymna the Weaver — `{1}{W}{B}` 2/2 Legendary Human Cleric with Lifelink.
//! "At the beginning of each of your postcombat main phases, you may pay X
//!  life, where X is the number of opponents that were dealt combat damage
//!  this turn. If you do, draw X cards."
//! Partner (not a usable KeywordAbility — see gap).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tymna the Weaver");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP keyword: "Partner" is not a usable KeywordAbility variant.
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "At the beginning of each of your postcombat main phases, you may
            //  pay X life, where X = number of opponents dealt combat damage
            //  this turn. If you do, draw X cards."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PostCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: tymna_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tymna_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay X life ... draw X cards" where X = number of opponents
    // dealt combat damage this turn — no accessor counts opponents damaged this
    // turn, and OptionalPaymentKind::Life takes a literal (not a dynamic X).
    Vec::new()
}
