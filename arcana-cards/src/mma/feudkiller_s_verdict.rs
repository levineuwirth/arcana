//! Feudkiller's Verdict — `{4}{W}{W}` Kindred Sorcery — Giant. "You
//! gain 10 life. Then if you have more life than an opponent, create
//! a 5/5 white Giant Warrior creature token."
//!
//! GAP: 'more life than an opponent' cross-player comparison isn't a
//! first-class helper. We resolve gain 10 life + unconditionally
//! create the Giant Warrior token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feudkiller's Verdict");
    let _giant = reg.interner_mut().intern("Giant");
    let _warrior = reg.interner_mut().intern("Warrior");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You gain 10 life. Then if you have more life than an opponent, create a 5/5 white Giant Warrior creature token.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let giant = reg.interner().lookup("Giant").expect("interned");
    let warrior = reg.interner().lookup("Warrior").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: giant,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: 'if you have more life than an opponent' conditional.
    vec![
        Effect::GainLife { player: entry.controller, amount: 10 },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
