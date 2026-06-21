//! Nathan Drake, Treasure Hunter — `{U}{B}{R}` 3/2 Legendary Creature — Human Rogue.
//! First strike.
//! "You may spend mana as though it were mana of any color to cast spells you
//! don't own or to activate abilities of permanents you control but don't
//! own." — a static mana-spending permission with no demonstrated hook; GAP'd.
//! "Whenever Nathan Drake attacks, exile the top card of each player's library.
//! You may cast a spell from among those cards." — exile-each-library-top plus
//! a cast-from-among-the-exiled-cards permission, which the demonstrated effect
//! API can't express; the SelfAttacks trigger structure is emitted but its
//! effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Nathan Drake, Treasure Hunter");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: static "spend mana as though any color to cast spells you don't own
    // / activate abilities of permanents you don't own" — no mana-permission hook.
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

fn on_attack(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile the top card of each player's library. You may cast a spell
    // from among those cards." — exile-each-library-top plus a cast-from-exile
    // permission is not expressible with the demonstrated effect API.
    Vec::new()
}
