//! Specter of Mortality — `{3}{B}{B}` 3/3 Specter with Flying.
//! When this enters, you may exile one or more creature cards from your graveyard.
//! When you do, each other creature gets -X/-X until end of turn, where X is the
//! number of cards exiled this way.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Specter of Mortality");
    let specter = reg.interner_mut().intern("Specter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(specter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_exile,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_exile(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // You may exile one or more creature cards from your graveyard.
    // GAP: the reflexive "when you do, each other creature gets -X/-X where X = number exiled"
    // cannot read the count chosen by ChooseAnyNumberFromZone, so the -X/-X payload is omitted.
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Graveyard(trig.controller),
        filter: ObjectFilter::creature(),
        action: PickAction::Exile,
    }]
}
