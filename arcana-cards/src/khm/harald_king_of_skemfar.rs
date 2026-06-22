//! Harald, King of Skemfar — `{1}{B}{G}` 3/2 Legendary Elf Warrior.
//!
//! Menace
//! When Harald enters, look at the top five cards of your library. You may
//! reveal an Elf, Warrior, or Tyvar card from among them and put it into
//! your hand. Put the rest on the bottom of your library in a random order.
//!
//! Decomposed as: a keyword line (Menace) plus one ETB dig. DigTopN looks
//! at the top 5, lets you take one matching card, and bottoms the rest in
//! random order. The takeable filter is an Elf-or-Warrior subtype OR; the
//! "Tyvar" (by-name) branch is a partial — the filter only matches the two
//! subtypes, so an off-subtype card named Tyvar would not be takeable.

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
    let name = reg.interner_mut().intern("Harald, King of Skemfar");
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
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
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
    // Takeable filter: Elf OR Warrior. (Partial: the by-name "Tyvar" branch
    // is not folded in — a card named Tyvar that is neither an Elf nor a
    // Warrior would not be takeable.)
    let mut subs = Vec::new();
    if let Some(elf) = reg.interner().lookup("Elf") {
        subs.push(elf);
    }
    if let Some(warrior) = reg.interner().lookup("Warrior") {
        subs.push(warrior);
    }
    let take_filter = ObjectFilter::new().with_subtypes_any(subs);
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(take_filter),
        rest: DigRest::BottomRandom,
    }]
}
