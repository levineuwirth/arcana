//! Lackey Recycler — artifact — Contraption (Un-set, no mana cost).
//! "Whenever you crank this Contraption, put target creature card
//! from your graveyard on top of your library."
//!
//! GAP: trigger — "whenever you crank this Contraption" (Contraption
//! decks / sprockets / cranking) is entirely unmodeled; the closest
//! physical analog `SelfBecomesTapped` is wired so the recycle effect
//! is at least exercised. The effect itself (graveyard creature card
//! to top of library) IS expressible.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lackey Recycler");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever you crank this Contraption" is
                // unmodeled (no crank event); SelfBecomesTapped is the
                // closest stand-in.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: recycle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…put target creature card from your graveyard on top of your
/// library."
fn recycle(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
