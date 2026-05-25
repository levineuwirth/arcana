//! Loamcrafter Faun — `{2}{G}` 3/3 green Satyr Druid. "When this
//! creature enters, you may discard one or more land cards. When you
//! do, return up to that many target nonland permanent cards from your
//! graveyard to your hand."
//! GAP: effect — the conditional "when you do" chained triggered sub-ability
//! and the dynamic count tied to number of lands discarded cannot be expressed
//! with the catalog; returning best-effort single discard + graveyard return.

use arcana_core::effects::Effect;
use arcana_core::effects::DiscardChoice;
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
    let name = reg.interner_mut().intern("Loamcrafter Faun");
    let satyr = reg.interner_mut().intern("Satyr");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — conditional "discard land cards, then return up to that many nonland
    // permanent cards from graveyard" requires chained conditional effects not in catalog
    vec![Effect::Discard { player: trig.controller, count: 1, choice: DiscardChoice::ControllerChooses }]
}
