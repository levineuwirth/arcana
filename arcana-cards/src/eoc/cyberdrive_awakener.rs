//! Cyberdrive Awakener — `{5}{U}` 4/4 blue Artifact Creature — Construct.
//!
//! Oracle:
//! * Flying
//! * Other artifact creatures you control have flying.  (static — GAP'd)
//! * When this creature enters, each noncreature artifact you control
//!   becomes a 4/4 artifact creature until end of turn.
//!
//! Flying is a base keyword. The "other artifact creatures … have flying"
//! line is a static keyword-anthem with no primitive in this surface, so it
//! is GAP'd. The ETB animation IS expressible: enumerate the controller's
//! noncreature artifacts, add the creature type and set base P/T to 4/4
//! until end of turn for each.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cyberdrive Awakener");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    // GAP: static "Other artifact creatures you control have flying." — no
    // continuous keyword-anthem primitive in the MultiAbilityCreature surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_animate_artifacts,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_animate_artifacts(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .without_types(TypeLine::CREATURE.into())
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::SetBasePT {
            target: id,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        });
    }
    vec![Effect::Sequence(effects)]
}
