//! Nick Valentine, Private Eye — `{2}{U}` 2/2 Legendary Artifact
//! Creature — Synth Detective (blue).
//!
//! * "Nick Valentine can't be blocked except by artifact creatures." —
//!   GAP: a "can't be blocked except by [filter]" static; the only
//!   evasion primitive is unconditional CantBeBlocked, which would be
//!   materially wrong here.
//! * "Whenever Nick Valentine or another artifact creature you control
//!   dies, you may investigate." — implemented as a death trigger that
//!   creates a Clue token. (The "may" is treated as always taking the
//!   beneficial option.)

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nick Valentine, Private Eye");
    let synth = reg.interner_mut().intern("Synth");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(synth);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // Investigate is a parametrized keyword not in the usable surface;
        // it appears here only as the dies-trigger payload below.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever Nick Valentine or another artifact creature you
            // control dies, you may investigate."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
