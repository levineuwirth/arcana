//! Ondu Cleric — `{1}{W}` 1/1 white Kor Cleric Ally. "Whenever this creature
//! or another Ally you control enters, you may gain life equal to the number
//! of Allies you control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ondu Cleric");
    let kor = reg.interner_mut().intern("Kor");
    let cleric = reg.interner_mut().intern("Cleric");
    let ally = reg.interner_mut().intern("Ally");
    let _ally2 = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(cleric);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_ally_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_ally_enters(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Ally").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::GainLife { player: trig.controller, amount: n }]
}
