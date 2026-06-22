//! Elder Brain — `{5}{B}{B}` 6/6 black Horror with Menace.
//!
//! * Menace.
//! * Whenever this creature attacks a player, exile all cards from that
//!   player's hand, then they draw that many cards. You may play lands and cast
//!   spells from among the exiled cards for as long as they remain exiled. If
//!   you cast a spell this way, you may spend mana as though it were mana of
//!   any color to cast it.
//!
//! Menace is expressed faithfully. The attack trigger is GAP'd: there is no
//! primitive that exiles a player's whole hand AND grants the attacker's
//! controller permission to play those exiled cards (with any-color mana), so
//! the entire payoff is unexpressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elder Brain");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_exile_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_exile_hand(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile all cards from that player's hand, then they draw that many
    // cards. You may play lands and cast spells from among the exiled cards ...
    // any color" — no primitive exiles a whole hand and grants the attacker
    // permission to play those exiled cards.
    Vec::new()
}
