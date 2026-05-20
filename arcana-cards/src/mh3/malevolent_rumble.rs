//! Malevolent Rumble — `{1}{G}` sorcery. "Reveal the top four cards
//! of your library. You may put a permanent card from among them into
//! your hand. Put the rest into your graveyard. Create a 0/1
//! colorless Eldrazi Spawn creature token with 'Sacrifice this token:
//! Add {C}.'"
//!
//! GAP: top-N reveal + select-a-permanent-into-hand + rest-to-yard is
//! not expressible in the catalog. The Eldrazi Spawn token (without
//! its sacrifice mana ability) is emitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malevolent Rumble");
    let _eldrazi = reg.interner_mut().intern("Eldrazi");
    let _spawn = reg.interner_mut().intern("Spawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Reveal the top four cards of your library. You may put a permanent card from among them into your hand. Put the rest into your graveyard. Create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\"".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi").expect("interned");
    let spawn = reg.interner().lookup("Spawn").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(spawn);
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: reveal-top-4-pick-permanent + rest-to-graveyard, and the
    // token's "Sacrifice: Add {C}" activated mana ability.
    vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }]
}
