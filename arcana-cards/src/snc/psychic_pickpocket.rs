//! Psychic Pickpocket — `{4}{U}` 3/2 blue Octopus Rogue.
//! "When this creature enters, it connives. When it connives this way, return
//! up to one target nonland permanent to its owner's hand."
//! GAP: keyword — Connive mechanic (draw + discard + conditional counter) not
//! in supported keyword set; emitting draw + discard as best-effort connive,
//! and bounce on target.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Psychic Pickpocket");
    let octopus = reg.interner_mut().intern("Octopus");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .without_types(TypeLine(TypeLine::LAND)),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: connive mechanic (draw + discard + conditional counter) not in catalog.
    let mut effects = vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard { player: trig.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ];
    if let Some(target) = trig.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnToHand { target: *id });
        }
    }
    effects
}
