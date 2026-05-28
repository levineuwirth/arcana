//! Smitten Swordmaster // Curry Favor — `{1}{B}` / `{B}` Adventure
//!
//! Creature: `{1}{B}` Creature — Human Knight (2/1)
//!   Lifelink.
//!
//! Adventure: `{B}` Sorcery — Curry Favor
//!   You gain X life and each opponent loses X life, where X is the number
//!   of Knights you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smitten Swordmaster");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Lifelink],
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Curry Favor");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "You gain X life and each opponent loses X life, where X is the number of Knights you control.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let knight_filter = script::subtype_filter(reg, "Knight")
        .controlled_by(arcana_core::targets::ControllerConstraint::You);
    let x = script::count_matching(state, &knight_filter, entry.controller);
    if x == 0 { return Vec::new(); }
    let opponents = script::opponents(state, entry.controller);
    let mut effects = vec![Effect::GainLife { player: entry.controller, amount: x }];
    effects.extend(opponents.into_iter().map(|p| Effect::LoseLife { player: p, amount: x }));
    effects
}
