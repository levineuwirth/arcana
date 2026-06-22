//! Cresting Mosasaurus — `{6}{U}{U}` 4/8 Dinosaur.
//! Emerge {6}{U}.
//! When this creature enters, if you cast it, return each non-Dinosaur creature
//! to its owner's hand.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cresting Mosasaurus");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(8)),
        // GAP: Emerge {6}{U} is an alternative casting cost — not an
        // expressible KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: bounce_non_dinosaurs,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn bounce_non_dinosaurs(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you cast it" — no cast-source intervening-if predicate is
    // available; the bounce fires on every entry (fidelity gap on the cast
    // gate only).
    let dinosaur = reg.interner().lookup("Dinosaur");
    let filter = ObjectFilter {
        not_subtypes: dinosaur.map(|d| vec![d]),
        ..ObjectFilter::creature()
    };
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::ReturnToHand { target: id })
        .collect()
}
