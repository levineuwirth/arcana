//! Forestfolk — `{2}{U}{G}` 2/2 green/blue Elf Wizard.
//! When Forestfolk enters the battlefield, you may search your library for a
//! basic land card, put it onto the battlefield tapped, then shuffle.
//! When Forestfolk leaves the battlefield, draw a card.
//!
//! The ETB tutors a basic land onto the battlefield tapped; the leave trigger
//! draws a card. Both abilities are faithful (the "you may" optionality of the
//! search resolves as a no-op if nothing matches / the player declines).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forestfolk");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: tutor_basic_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tutor_basic_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter,
        tapped: true,
    }]
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
