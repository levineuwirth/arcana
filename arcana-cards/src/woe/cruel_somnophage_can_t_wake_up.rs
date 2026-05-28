//! Cruel Somnophage // Can't Wake Up — `{1}{B}` // `{1}{U}` black Adventure creature.
//! Creature: *//* Nightmare. "P/T = number of creature cards in all graveyards."
//! Adventure (Can't Wake Up — Sorcery): Target player mills four cards.
//! GAP: "*/*" dynamic P/T (creature cards in all graveyards) — not in PtValue (only Fixed); using 0/0 as placeholder.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Somnophage");
    let adv_name = reg.interner_mut().intern("Can't Wake Up");
    let nightmare_sub = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        // GAP: *//* P/T (creature cards in all graveyards) — using 0/0 as placeholder
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Target player mills four cards.".into(), target_requirements: vec![TargetRequirement::target_player()], modal: None, effect: cant_wake_up_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn cant_wake_up_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Mill { player: *p, count: 4 }]
}
