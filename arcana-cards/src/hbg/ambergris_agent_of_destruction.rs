//! Ambergris, Agent of Destruction — `{2}{R}{R}` 4/3 Legendary Dwarf Cleric.
//! "Haste"
//! "Whenever Ambergris attacks, you may discard your hand and draw two cards.
//! If you do, Ambergris deals X damage to each opponent, where X is the number
//! of cards you've discarded this turn."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ambergris, Agent of Destruction");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "you may discard your hand and draw two cards" — modeled mandatorily.
    // GAP: the "you may … if you do" optional gate is not separately modeled.
    let hand = script::hand_size(state, trig.controller);
    let mut effects = vec![
        Effect::Discard {
            player: trig.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 2,
        },
    ];
    // X = number of cards discarded this turn (after the discard above).
    let x = script::cards_discarded_this_turn(state, trig.controller) + hand;
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(opp),
            amount: x,
        });
    }
    effects
}
