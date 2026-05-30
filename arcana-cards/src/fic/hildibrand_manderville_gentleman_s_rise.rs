//! Hildibrand Manderville // Gentleman's Rise
//!
//! Front: {1}{W} Legendary Creature — Human Detective 2/2.
//! Creature tokens you control get +1/+1. (GAP: continuous static pump not modeled)
//! When Hildibrand Manderville dies, you may cast it from your graveyard as an Adventure
//! until the end of your next turn. (GAP: graveyard-cast-as-adventure triggered ability not modeled)
//!
//! Adventure face: "Gentleman's Rise" {2}{B} Instant.
//! Create a 2/2 black Zombie creature token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hildibrand Manderville");
    let sub_human = reg.interner_mut().intern("Human");
    let sub_detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_detective);

    // Pre-intern Zombie subtype for use in token creation at resolve time.
    let _zombie_pre = reg.interner_mut().intern("Zombie");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // Adventure face: Gentleman's Rise
    let adv_name = reg.interner_mut().intern("Gentleman's Rise");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid adv cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a 2/2 black Zombie creature token.".into(),
        target_requirements: vec![],
        modal: None,
        effect: gentleman_rise_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    // GAP: "Creature tokens you control get +1/+1" continuous static pump not modeled.
    // GAP: "When Hildibrand Manderville dies, you may cast it from your graveyard as an Adventure"
    // triggered ability not modeled.
    reg.register(CardDefinition::new(name, chars).with_adventure(adventure))
}

fn gentleman_rise_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie_sym = reg.interner().lookup("Zombie");
    // TokenDefinition.name is SmallString (u32 interner id); default to 0 if not found.
    let zombie_name_id = zombie_sym.unwrap_or(0);
    let mut zombie_subtypes = SubtypeSet::default();
    if let Some(s) = zombie_sym {
        zombie_subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: entry.controller,
        token: TokenDefinition {
            name: zombie_name_id,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: zombie_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
