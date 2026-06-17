//! Cephalid Facetaker — `{2}{U}` 1/4 Octopus Rogue.
//! "This creature can't be blocked." (static)
//! "At the beginning of combat on your turn, you may have this creature
//! become a copy of another target creature until end of turn, except
//! it's 1/4 and has 'This creature can't be blocked.'"
//!
//! The combat trigger is wired with a creature target, but its body is
//! GAP'd: CopyPermanent MINTS A NEW TOKEN copy — there is no "have this
//! creature BECOME a copy of another creature" effect in the API. The
//! leading can't-be-blocked static likewise has no static-ability hook
//! in this card class.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cephalid Facetaker");
    let octopus = reg.interner_mut().intern("Octopus");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "This creature can't be blocked" — no static-ability hook
    // in this card class (CantBeBlocked is an effect, applied via abilities).

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: become_copy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn become_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "have this creature become a copy of another target creature
    // until end of turn, except it's 1/4 and can't be blocked" — no
    // become-a-copy effect (CopyPermanent only mints a new token copy).
    Vec::new()
}
