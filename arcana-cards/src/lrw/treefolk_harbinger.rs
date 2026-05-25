//! Treefolk Harbinger — `{G}` 0/3 green creature (Treefolk Druid).
//! "When this creature enters, you may search your library for a Treefolk
//! or Forest card, reveal it, then shuffle and put that card on top."
//!
//! GAP: TutorToHand with "Treefolk or Forest" disjunction — the engine's
//! TutorToHand uses a single ObjectFilter; no OR-filter across subtypes and
//! land type is available. Using TutorToHand with creature Treefolk filter
//! as best effort; Forest-land half is not expressible.
//! GAP: "put on top" — TutorToHand puts into hand; PutOnTopOfLibrary would
//! need a separate step. Using TutorToHand as closest available.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Treefolk Harbinger");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
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
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: OR-filter (Treefolk creature OR Forest land) not supported;
    // using Treefolk creature subtype filter only.
    let filter = script::subtype_filter(reg, "Treefolk");
    vec![Effect::TutorToHand { player: trig.controller, filter, reveal: true }]
}
