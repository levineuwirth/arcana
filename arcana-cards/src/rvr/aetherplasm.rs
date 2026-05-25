//! Aetherplasm — `{2}{U}{U}` 1/1 blue Illusion creature.
//! "Whenever this creature blocks a creature, you may return this creature to its owner's
//! hand. If you do, you may put a creature card from your hand onto the battlefield
//! blocking that creature."
//! The second part (put creature card from hand onto battlefield blocking) is not
//! expressible with the catalog. Only the ReturnToHand portion is modelled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aetherplasm");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: blocks_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blocks_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you do, put a creature card from your hand onto the battlefield blocking"
    // is not expressible with the catalog.
    vec![Effect::ReturnToHand { target: trig.source }]
}
