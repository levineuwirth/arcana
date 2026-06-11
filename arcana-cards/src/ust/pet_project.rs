//! Pet Project — artifact — Contraption (no mana cost).
//! "Whenever you crank this Contraption, put target creature card from
//! an opponent's graveyard onto the battlefield under your control."
//!
//! GAP: trigger — "Whenever you crank this Contraption" (the
//! crank/sprocket Contraption mechanic is unmodeled); wired on the
//! closest available condition, `SelfEntersBattlefield`.

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
    let name = reg.interner_mut().intern("Pet Project");
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
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "Whenever you crank this Contraption" has no
            // TriggerCondition variant; SelfEntersBattlefield is the
            // closest available stand-in.
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: steal_from_graveyard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "an opponent's graveyard" — the Card target zone is
            // declared as Zone::Graveyard(0) per the catalog shape; an
            // opponent-graveyard constraint is not expressible.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

/// "…put target creature card from an opponent's graveyard onto the
/// battlefield under your control."
fn steal_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "under your control" — ReturnFromGraveyardToBattlefield puts
    // the card onto the battlefield per the engine's default owner
    // routing; the control-stealing rider is a documented fidelity gap.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
