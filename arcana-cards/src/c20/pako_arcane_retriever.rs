//! Pako, Arcane Retriever — `{3}{R}{G}` 3/3 Legendary Creature —
//! Elemental Dog with Haste.
//! "Partner with Haldan, Avid Arcanist."
//! "Whenever Pako attacks, exile the top card of each player's library and
//! put a fetch counter on each of them. Put a +1/+1 counter on Pako for
//! each noncreature card exiled this way."
//!
//! Haste is expressed. Partner / Partner with are not in the usable
//! keyword surface. The attack trigger is wired but its payload is GAP'd.

// GAP: "Partner with Haldan, Avid Arcanist" / "Partner" — not in the
//      usable keyword surface (no KeywordAbility::Partner variant).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Pako, Arcane Retriever");
    let elemental = reg.interner_mut().intern("Elemental");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(dog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_exile_and_grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_exile_and_grow(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of each player's library and put a fetch
    //      counter on each of them; put a +1/+1 counter on Pako for each
    //      noncreature card exiled this way" — no effect for exiling the
    //      top card of a library, no way to mark/count those exiled cards.
    Vec::new()
}
