//! Cait, Cage Brawler — `{R}{G}` 1/1 Legendary Human Warrior.
//!
//! Oracle:
//! * During your turn, Cait has indestructible. — a conditional static
//!   keyword grant; not expressible on this card class, so it is GAP'd.
//! * Whenever Cait attacks, you and defending player each draw a card, then
//!   discard a card. Put two +1/+1 counters on Cait if you discarded the
//!   card with the greatest mana value among those cards or tied for
//!   greatest. — `SelfAttacks` trigger. The draw-then-discard for both you
//!   and the defending player is modeled; the conditional "+2 counters if
//!   you discarded the greatest-mv card" rider is GAP'd (no way to compare
//!   the discarded cards' mana values).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cait, Cage Brawler");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static — "During your turn, Cait has indestructible."

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_loot(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ];
    if let Some(defender) = trig.defending_player() {
        effects.push(Effect::DrawCards {
            player: defender,
            count: 1,
        });
        effects.push(Effect::Discard {
            player: defender,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    // GAP: rider — "Put two +1/+1 counters on Cait if you discarded the
    // card with the greatest mana value among those cards." No primitive
    // compares the discarded cards' mana values.
    vec![Effect::Sequence(effects)]
}
