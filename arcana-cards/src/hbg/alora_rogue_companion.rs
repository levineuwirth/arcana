//! Alora, Rogue Companion — `{3}{U}` 3/3 Legendary Creature — Halfling Rogue.
//!
//! Specialize {2}
//! Whenever you attack, up to one target attacking creature can't be
//!   blocked this turn. At the beginning of the next end step, return that
//!   creature to its owner's hand.
//!
//! Decomposition: the attack trigger → one `TriggeredAbilityDef`. It
//! targets up to one attacking creature, makes it unblockable
//! (`Effect::CantBeBlocked`), and schedules a return-to-hand at the next
//! end step (`Effect::DelayedAction` with `ReturnToHand`). Specialize {2}
//! is NOT in the usable keyword surface (the specialize action — paying a
//! cost to transform into a colored back face — is unmodeled), so
//! `keywords: vec![]`. "Whenever you attack" has no exact condition; it is
//! modeled as `CreatureAttacks` over a creature you control (a documented
//! over-fire-per-attacker fidelity gap).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alora, Rogue Companion");
    let halfling = reg.interner_mut().intern("Halfling");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword "Specialize {2}" — the specialize transform action is
        // not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        // GAP (trigger fidelity): "Whenever you attack" has no exact
        // condition; modeled as CreatureAttacks for a creature you control,
        // which over-fires per attacker.
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: unblockable_then_bounce,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn unblockable_then_bounce(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::CantBeBlocked {
            target: *id,
            duration: Duration::EndOfTurn,
        },
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        },
    ]
}
