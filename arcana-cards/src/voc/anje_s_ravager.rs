//! Anje's Ravager — `{2}{R}` 3/3 Creature — Vampire Berserker (red).
//!
//! * "This creature attacks each combat if able." — a static combat-
//!   restriction with no trigger/cost; not expressible. GAP'd.
//! * "Whenever this creature attacks, discard your hand, then draw three
//!   cards." — SelfAttacks trigger: discard the whole hand, then draw 3.
//! * Madness {1}{R} — Madness is not a usable KeywordAbility variant; GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anje's Ravager");
    let vampire = reg.interner_mut().intern("Vampire");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(berserker);

    // GAP: "attacks each combat if able" is a static combat restriction;
    // GAP: Madness is not a usable KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfAttacks,
        intervening_if: None,
        effect: discard_hand_draw_three,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }))
}

fn discard_hand_draw_three(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let hand = script::hand_size(state, trig.controller);
    vec![
        Effect::Discard {
            player: trig.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 3,
        },
    ]
}
