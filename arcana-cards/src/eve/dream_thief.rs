//! Dream Thief — `{2}{U}` 2/1 Faerie Rogue.
//!
//! * Flying.
//! * When this creature enters, draw a card if you've cast another blue
//!   spell this turn. (Dream Thief itself is a blue spell cast this
//!   turn, so "another" means the total count of blue spells you cast
//!   this turn is at least two.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dream Thief");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_draw_if_blue,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_draw_if_blue(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let blue_spells = script::spells_cast_this_turn(
        state,
        &ObjectFilter::new().with_colors(ColorSet::blue()),
        trig.controller,
    );
    if blue_spells >= 2 {
        vec![Effect::DrawCards {
            player: trig.controller,
            count: 1,
        }]
    } else {
        Vec::new()
    }
}
