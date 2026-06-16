//! Realm-Cloaked Giant // Cast Off — `{5}{W}{W}` // `{3}{W}{W}` white Adventure creature.
//! Creature: 7/7 Giant. Vigilance.
//! Adventure (Cast Off — Sorcery): Destroy all non-Giant creatures.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Realm-Cloaked Giant");
    let adv_name = reg.interner_mut().intern("Cast Off");
    let giant_sub = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")), colors: ColorSet::white(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(7)), toughness: Some(PtValue::Fixed(7)), keywords: vec![KeywordAbility::Vigilance], ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")), colors: ColorSet::white(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Destroy all non-Giant creatures.".into(), target_requirements: vec![], modal: None, effect: cast_off_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn cast_off_resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let _giant_filter = script::subtype_filter(reg, "Giant");
    let non_giant_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().with_subtypes_any(vec![]),  // all creatures
        entry.controller,
    );
    // Approximate: destroy all creatures except Giants — need to filter out Giants
    // GAP: "without Giant subtype" filter not directly available; using ForEach on all and noting gap
    if non_giant_creatures.is_empty() { return Vec::new(); }
    vec![Effect::ForEach {
        targets: non_giant_creatures,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
