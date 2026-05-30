//! Kessig Naturalist // Lord of the Ulvenwald
//!
//! Front (Kessig Naturalist, {R}{G}, 2/2 Human Werewolf):
//!   Whenever this creature attacks, add {R} or {G}.
//!   (GAP: "{R} or {G}" is a choice; modeled as adding {R} only.)
//!   (GAP: "until end of turn, you don't lose this mana as steps and phases end" not modeled.)
//!   Daybound (GAP: day/night cycle not modeled.)
//!
//! Back (Lord of the Ulvenwald, 5/5 Werewolf):
//!   Other Wolves and Werewolves you control get +1/+1. (GAP: back-face-only static anthem not modeled.)
//!   Whenever this creature attacks, add {R} or {G}. (shared trigger — see above)
//!   Nightbound (GAP: day/night cycle not modeled.)

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kessig Naturalist");
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Lord of the Ulvenwald");
    let werewolf2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Whenever this creature attacks, add {R} (approximation of {R} or {G})
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: add_mana_on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: Daybound/Nightbound day-night cycle not modeled.
            // GAP: back-face-only static anthem (other Wolves/Werewolves +1/+1) not modeled.
    )
}

fn add_mana_on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: {R} or {G} choice — modeling as {R} only.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}
