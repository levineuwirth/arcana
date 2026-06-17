//! Dragonologist — `{2}{U}` 1/3 Human Wizard.
//!
//! When this creature enters, look at the top six cards of your
//! library; you may reveal an instant, sorcery, or Dragon card from
//! among them and put it into your hand; put the rest on the bottom of
//! your library in a random order.
//! "Untapped Dragons you control have hexproof." (static — GAP'd)
//!
//! The ETB dig is modeled with `DigTopN`. The takeable filter expresses
//! the instant-or-sorcery clause; the "or Dragon" disjunction across a
//! subtype cannot be OR'd with a type filter in one `ObjectFilter`, so
//! that clause is a documented fidelity GAP.

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Dragonologist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP (static): "Untapped Dragons you control have hexproof" — continuous ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: dig_six,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dig_six(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP (fidelity): the "or Dragon" disjunction can't be OR'd with a type filter in one ObjectFilter.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(
            ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
        ),
        rest: DigRest::BottomRandom,
    }]
}
