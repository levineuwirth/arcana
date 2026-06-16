//! Gonti, Lord of Luxury — `{2}{B}{B}` 2/3 Legendary Aetherborn Rogue.
//! Deathtouch.
//! "When Gonti enters, look at the top four cards of target opponent's
//! library, exile one of them face down, then put the rest on the bottom of
//! that library in a random order. You may cast that card for as long as it
//! remains exiled, and mana of any type can be spent to cast that spell."

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
    let name = reg.interner_mut().intern("Gonti, Lord of Luxury");
    let aetherborn = reg.interner_mut().intern("Aetherborn");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aetherborn);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: gonti_etb,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gonti_etb(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at the top four of target opponent's library, exile one face
    // down, rest on bottom in random order; you may cast that card for as long
    // as it remains exiled with mana of any type" — no exile-from-opponent-
    // library-and-cast primitive available (DigTopN puts to own hand, not a
    // cast-from-exile permission across the opponent's deck).
    Vec::new()
}
