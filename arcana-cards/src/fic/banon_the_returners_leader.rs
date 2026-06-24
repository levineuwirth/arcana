//! Banon, the Returners' Leader — `{R}{W}` 1/3 Legendary Human Cleric Rebel.
//! Pray — Once during each of your turns, you may cast a creature spell from
//! among cards in your graveyard that were put there from anywhere other than
//! the battlefield this turn.
//! Whenever you attack, you may pay {1} and discard a card. If you do, draw a
//! card.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Banon, the Returners' Leader");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let rebel = reg.interner_mut().intern("Rebel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    subtypes.0.insert(rebel);

    // GAP: "Pray — Once during each of your turns, you may cast a creature spell
    // from among cards in your graveyard …" is an alternative cast-permission
    // (cast-from-graveyard with a same-turn-zone-change condition); no Effect
    // grants graveyard casting permission.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "Whenever you attack" (any attack declaration) has
            // no exact variant; SelfAttacks fires only when Banon itself
            // attacks (a subset).
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {1} and discard a card. If you do, draw a card." — the
    // optional cost is a COMPOUND of mana AND a card discard (both paid
    // together). OptionalPaymentKind carries a single cost, so a "pay {1} AND
    // discard a card" gate cannot be expressed.
    Vec::new()
}
