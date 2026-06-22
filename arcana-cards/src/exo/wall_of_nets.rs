//! Wall of Nets — `{1}{W}{W}` 0/7 Wall with Defender.
//! Defender.
//! At end of combat, exile all creatures blocked by this creature.
//! When this creature leaves the battlefield, return all cards exiled with it
//! to the battlefield under their owners' control.
//!
//! Defender is a base characteristic. The end-of-combat exile ("all creatures
//! blocked by this creature") needs the set of creatures this creature blocked
//! this combat — no script accessor exposes it — and its companion leave
//! trigger must return exactly the cards exiled BY this object (a linked
//! exile-zone association the effect surface cannot create). Both triggers'
//! effects are GAP'd; the shells are wired with the closest conditions.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Nets");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::EndCombat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: gap_exile_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: gap_return_exiled,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gap_exile_blocked(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile all creatures blocked by this creature." No script accessor
    // exposes the set of creatures this creature blocked this combat.
    Vec::new()
}

fn gap_return_exiled(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return all cards exiled with it." Requires returning exactly the
    // cards exiled by this object — there is no linked exile-zone association
    // primitive for the bespoke end-of-combat exile above.
    Vec::new()
}
