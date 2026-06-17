//! Rehearsed Debater — `{2}{W}` 3/3 Djinn Bard with Vigilance.
//!
//! Repartee — "Whenever you cast an instant or sorcery spell that targets
//! a creature, this creature gets +1/+1 until end of turn."
//!
//! GAP (partial trigger filter): the "that targets a creature" qualifier
//! on the cast spell isn't expressible in `ObjectFilter`, so the trigger
//! fires on any instant/sorcery you cast. The +1/+1 payload is faithful.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rehearsed Debater");
    let djinn = reg.interner_mut().intern("Djinn");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(bard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "that targets a creature" qualifier not expressible.
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                    TypeLine::INSTANT | TypeLine::SORCERY,
                ))),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: pump_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
