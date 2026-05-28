//! Kellan, the Fae-Blooded // Birthright Boon — `{2}{R}` / `{1}{W}` Adventure
//!
//! Creature: `{2}{R}` Legendary Creature — Human Faerie (2/2)
//!   Double strike.
//!   Other creatures you control get +1/+0 for each Aura and Equipment
//!   attached to Kellan. (GAP: static buff based on attachment count deferred.)
//!
//! Adventure: `{1}{W}` Sorcery — Birthright Boon
//!   Search your library for an Aura or Equipment card, reveal it, put it
//!   into your hand, then shuffle.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kellan, the Fae-Blooded");
    let human_sub = reg.interner_mut().intern("Human");
    let faerie_sub = reg.interner_mut().intern("Faerie");
    let _equip_sub = reg.interner_mut().intern("Equipment");
    let _aura_sub = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(faerie_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::DoubleStrike],
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Birthright Boon");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Search your library for an Aura or Equipment card, reveal it, put it into your hand, then shuffle.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // Search for Aura or Equipment — use subtype OR filter
    let aura_sub = reg.interner().lookup("Aura").unwrap_or_default();
    let equip_sub = reg.interner().lookup("Equipment").unwrap_or_default();
    let filter = ObjectFilter::new()
        .with_subtypes_any(vec![aura_sub, equip_sub]);
    vec![Effect::TutorToHand { player: entry.controller, filter, reveal: true }]
}
