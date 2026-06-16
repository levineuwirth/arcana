//! Marchesa, the Black Rose — `{1}{U}{B}{R}` 3/3 Legendary Human Wizard with
//! Dethrone.
//!
//! Oracle:
//! * Dethrone.
//! * Other creatures you control have dethrone. (static grant — GAP)
//! * Whenever a creature you control with a +1/+1 counter on it dies, return
//!   that card to the battlefield under your control at the beginning of the
//!   next end step.
//!
//! The trigger watches creatures you control dying; the "+1/+1 counter on it"
//! gate isn't part of the demonstrated `ObjectFilter` surface (noted), and the
//! "at the beginning of the next end step" delay isn't expressible as a
//! DelayedAction — the reanimation is applied immediately as a fidelity gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marchesa, the Black Rose");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Dethrone],
        ..Default::default()
    };

    // GAP: static "Other creatures you control have dethrone."
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // The "+1/+1 counter on it" gate isn't in the demonstrated filter surface.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: reanimate_counter_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn reanimate_counter_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    // Fidelity gap: returned immediately rather than at the next end step.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}
