//! A-Harald, King of Skemfar — `{1}{B}{G}` 3/2 Legendary Elf Warrior with
//! Menace.
//!
//! Oracle:
//! * Menace
//! * "When Harald enters, look at the top seven cards of your library. You may
//!   reveal an Elf, Warrior, or Tyvar card from among them and put it into
//!   your hand. Put the rest on the bottom of your library in a random order."
//!
//! Partial: modeled as `DigTopN` over the top 7 with an Elf-or-Warrior subtype
//! filter, rest to the bottom in a random order. "or Tyvar" (a card-name
//! match) cannot be OR'd into the same subtype filter — GAP'd.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Harald, King of Skemfar");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut syms = Vec::new();
    if let Some(elf) = reg.interner().lookup("Elf") {
        syms.push(elf);
    }
    if let Some(warrior) = reg.interner().lookup("Warrior") {
        syms.push(warrior);
    }
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 7,
        filter: Some(ObjectFilter::default().with_subtypes_any(syms)),
        rest: DigRest::BottomRandom,
    }]
}
