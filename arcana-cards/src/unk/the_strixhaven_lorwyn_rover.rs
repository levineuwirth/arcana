//! The Strixhaven-Lorwyn Rover — `{4}` 3/3 Legendary Artifact Creature — Construct.
//! "Before the game begins, choose a Strixhaven college and a Lorwyn creature type.
//!  This card has the color identity of the college chosen this way.
//!  Whenever you cast an instant or sorcery spell that's both the chosen college's
//!  colors or a creature spell of the chosen creature type, copy it. You may choose
//!  new targets for the copy."
//!
//! The pregame college/creature-type choice and the resulting dynamic color identity
//! are not modeled, so the cast-copy trigger's filter depends on data the engine
//! cannot derive; the trigger is recorded with a best-effort spell-cast watcher and
//! its effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Strixhaven-Lorwyn Rover");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        // GAP: color identity is the pregame-chosen college's colors; unmodeled.
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_cast(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "copy it" gates on a pregame-chosen Strixhaven college's colors AND a
    // pregame-chosen Lorwyn creature type — neither the pregame choice nor a
    // "copy the triggering spell" handle is expressible. (Effect::CopySpell needs a
    // chosen spell-object target; the triggering spell id is not surfaced here.)
    Vec::new()
}
