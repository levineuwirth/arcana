//! Scavenger Regent // Exude Toxin — `{3}{B}` / `{X}{B}{B}` Adventure
//!
//! Creature: `{3}{B}` Creature — Dragon (4/4)
//!   Flying.
//!   Ward—Discard a card. (Non-mana ward — GAP: emitting keywords: vec![].)
//!
//! Adventure: `{X}{B}{B}` Sorcery — Exude Toxin (Omen)
//!   Each non-Dragon creature gets -X/-X until end of turn.
//!   (GAP: X from cast cost not accessible at resolve time; emitting -1/-1
//!   to all non-Dragon creatures as approximation.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scavenger Regent");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flying],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Exude Toxin");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{X}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Each non-Dragon creature gets -X/-X until end of turn.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: X not accessible; emitting -1/-1 to all creatures as approximation
    let filter = ObjectFilter::creature();
    let ids = script::ids_matching(state, &filter, entry.controller);
    ids.into_iter()
        .map(|id| Effect::Pump { target: id, power: -1, toughness: -1, duration: Duration::EndOfTurn, keywords: vec![] })
        .collect()
}
