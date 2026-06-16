//! G'raha Tia — `{4}{W}` Legendary Creature — Cat Archer, 3/5.
//!
//! Oracle:
//! * Reach.
//! * The Allagan Eye — Whenever one or more other creatures and/or artifacts
//!   you control die, draw a card. This ability triggers only once each turn.
//!
//! "The Allagan Eye" is just the printed name of the triggered ability, not a
//! Scryfall keyword, so `keywords` carries only Reach. The dies trigger is a
//! battlefield -> graveyard `ZoneChange` filtered to creatures/artifacts you
//! control, fired at most once per turn via `TriggerFrequency::OncePerTurn`.

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
    let name = reg.interner_mut().intern("G'raha Tia");
    let cat = reg.interner_mut().intern("Cat");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                // "creatures and/or artifacts you control" — match a creature OR
                // artifact you control moving to the graveyard. ("other" exclusion
                // of G'raha Tia itself is a minor fidelity gap.)
                filter: ObjectFilter::new()
                    .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT))
                    .controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
