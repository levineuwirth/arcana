//! Purestrain Genestealer — `{2}{G}` 1/1 Tyranid.
//!
//! * "This creature enters with two +1/+1 counters on it." —
//!   `EntersWithSpec::Counters { kind: PlusOnePlusOne, count: 2 }`.
//! * "Vanguard Species — Whenever this creature attacks, you may remove a
//!   +1/+1 counter from it. If you do, search your library for a basic land
//!   card, put it onto the battlefield tapped, then shuffle." — the
//!   `SelfAttacks` trigger is wired, but GAP the effect: the "you may remove
//!   a +1/+1 counter from it" optional cost gating the tutor is not
//!   expressible (`OptionalPayment` only supports Mana/Life costs, not
//!   counter removal), and emitting the free search would be materially
//!   wrong.
//!
//! (Scryfall lists "Vanguard Species" as a keyword, but it is an ability
//! word naming the attack trigger — not a usable `KeywordAbility` variant.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Purestrain Genestealer");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::PlusOnePlusOne,
                count: 2,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: vanguard_species,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn vanguard_species(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the optional "remove a +1/+1 counter from it" cost gating the
    // basic-land search is not expressible (OptionalPayment only supports
    // Mana/Life). Emitting the free tutor would be materially wrong.
    Vec::new()
}
