//! Hamza, Might of the Yathan — `{2}{W}{B}{G}` 5/6 Legendary Elephant Warrior.
//! "Whenever a creature you control is turned face up, it endures X, where X is that
//! creature's toughness."
//! "Landfall — Whenever a land you control enters, seek a creature card and manifest it."
//!
//! GAP: the "turned face up" trigger has no TriggerCondition (no TurnedFaceUp event) and
//! Endure is not a modeled mechanic — the WHOLE first ability is unimplementable, omitted.
//! GAP: Seek/Endure/Landfall are not KeywordAbility variants → keywords vec empty.
//! Landfall is modeled via ZoneChange (land you control enters → Battlefield). "Seek a
//! creature card" has no Seek effect; the closest expressible payoff is Manifest (top card
//! of library becomes a face-down 2/2). GAP: seek-by-criteria → just manifests the top card.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Hamza, Might of the Yathan");
    let elephant = reg.interner_mut().intern("Elephant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Seek, Endure, and Landfall are not KeywordAbility variants.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "Whenever a creature you control is turned face up, it endures X" — there is no
    // TurnedFaceUp TriggerCondition and no Endure effect; the whole first ability is omitted.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Landfall — "Whenever a land you control enters".
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::new()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: on_landfall,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_landfall(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek a creature card" has no Seek effect; Manifest the top card instead.
    vec![Effect::Manifest {
        player: trig.controller,
    }]
}
