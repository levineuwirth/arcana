//! Miasma Demon — `{4}{B}{B}` 5/4 Demon with Flying.
//! "When this creature enters, you may discard any number of cards.
//! When you do, up to that many target creatures each get -2/-2 until
//! end of turn." — the "discard any number" half is expressed via
//! ChooseAnyNumberFromZone; the reflexive "up to that many target
//! creatures get -2/-2" cannot be linked to the discard count → GAP'd.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::effects::KeywordAbility;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miasma Demon");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_discard_any_number,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_discard_any_number(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "When you do, up to that many target creatures each get
    // -2/-2 until end of turn" — the reflexive trigger's target count
    // is the number of cards discarded, which cannot be linked to the
    // ChooseAnyNumberFromZone pick. Only the discard is expressed.
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Hand(trig.controller),
        filter: ObjectFilter::default(),
        action: PickAction::Discard,
    }]
}
