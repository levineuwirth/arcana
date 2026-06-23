//! Ultramarines Honour Guard — `{3}{W}` 2/2 Astartes Warrior.
//!
//! * Squad {2} (… When this creature enters, create that many tokens that are
//!   copies of it.)
//!   // GAP: Squad is not in the usable KeywordAbility surface; the additional
//!   variable cost and the "create that many copies" ETB are gated on a squad
//!   count that has no expressible form. Unmodeled.
//! * Other creatures you control get +1/+1.
//!   Wired via a SelfEntersBattlefield trigger installing a
//!   `ContinuousEffect::filtered_pump` over creatures you control, +1/+1,
//!   lasting while this creature is on the battlefield. (The "other" qualifier
//!   is a documented minor fidelity gap — this creature matches the
//!   base-characteristics filter, so it self-includes.)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Ultramarines Honour Guard");
    let astartes = reg.interner_mut().intern("Astartes");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Squad {2} — not in the usable KeywordAbility surface; the additional
    // variable cost and "create that many copies" ETB are not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            // "Other creatures you control get +1/+1."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_anthem(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
