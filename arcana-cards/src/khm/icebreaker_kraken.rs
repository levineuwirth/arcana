//! Icebreaker Kraken — `{10}{U}{U}` 8/8 Snow Creature — Kraken.
//! "Affinity for snow lands" (cost reduction — not a usable keyword, GAP'd).
//! "When this creature enters, artifacts and creatures target opponent
//!  controls don't untap during that player's next untap step."
//! "Return three snow lands you control to their owner's hand: Return this
//!  creature to its owner's hand."
//!
//! Affinity is not an expressible keyword (cost reduction) — keywords vec is
//! empty. The ETB don't-untap effect has no primitive and is GAP'd (the
//! trigger targets an opponent but the effect can't be expressed). The
//! activated ability's cost ("return three snow lands you control") is not an
//! expressible ActivationCost field, so that ability is GAP'd entirely.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Icebreaker Kraken");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    // GAP: activated ability "Return three snow lands you control to their
    // owner's hand: Return this creature to its owner's hand." — the return-
    // permanents-as-cost is not an expressible ActivationCost field.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_lock_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn etb_lock_untap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "artifacts and creatures target opponent controls don't untap
    // during that player's next untap step" — no don't-untap-next-step
    // primitive available.
    Vec::new()
}
