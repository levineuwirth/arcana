//! Rapid Prototyper — artifact, subtype Contraption.
//! "Whenever you crank this Contraption, create an X/X colorless
//! Construct artifact creature token, where X is the number of
//! artifacts you control."
//!
//! GAP: trigger — "Whenever you crank this Contraption" (the
//! Contraption crank mechanic is not modeled; no crank
//! TriggerCondition exists). The closest available self-event
//! condition `SelfBecomesTapped` is used as a placeholder so the
//! dynamic token effect is still exercised.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rapid Prototyper");
    let contraption = reg.interner_mut().intern("Contraption");
    let _construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "Whenever you crank this Contraption" has no
            // TriggerCondition (crank is unmodeled); SelfBecomesTapped is
            // the closest placeholder.
            trigger_condition: TriggerCondition::SelfBecomesTapped,
            intervening_if: None,
            effect: crank_make_construct,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        },
    ))
}

fn crank_make_construct(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let construct = reg.interner().lookup("Construct").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: construct,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(x as i32)),
            toughness: Some(PtValue::Fixed(x as i32)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
