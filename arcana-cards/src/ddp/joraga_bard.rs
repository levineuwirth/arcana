//! Joraga Bard — `{3}{G}` 1/4 green Elf Rogue Bard Ally.
//! "Whenever this creature or another Ally you control enters, you may have
//! Ally creatures you control gain vigilance until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    let name = reg.interner_mut().intern("Joraga Bard");
    let elf = reg.interner_mut().intern("Elf");
    let rogue = reg.interner_mut().intern("Rogue");
    let bard = reg.interner_mut().intern("Bard");
    let _ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(rogue);
    subtypes.0.insert(bard);
    subtypes.0.insert(_ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // ZoneChange: any Ally you control enters (including self).
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

fn on_ally_enters(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Grant Vigilance to all Ally creatures you control.
    let ally_filter = script::subtype_filter(reg, "Ally")
        .controlled_by(ControllerConstraint::You);
    let targets = script::ids_matching(state, &ally_filter, trig.controller);
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        }),
    }]
}
