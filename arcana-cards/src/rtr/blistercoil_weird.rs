//! Blistercoil Weird — `{U/R}` 1/1 blue/red Weird.
//! "Whenever you cast an instant or sorcery spell, this creature gets
//! +1/+1 until end of turn. Untap it."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Blistercoil Weird");
    let weird = reg.interner_mut().intern("Weird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weird);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U/R}").expect("valid cost")),
        colors: ColorSet(ColorSet::BLUE | ColorSet::RED),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types_any(
                        TypeLine(TypeLine::INSTANT | TypeLine::SORCERY),
                    )),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: spell_pump_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn spell_pump_untap(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Pump {
            target: trig.source,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Untap { target: trig.source },
    ]
}
