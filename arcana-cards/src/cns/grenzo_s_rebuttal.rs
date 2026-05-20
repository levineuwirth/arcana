//! Grenzo's Rebuttal — `{4}{R}{R}` sorcery, "Create a 4/4 red Ogre
//! creature token. Starting with you, each player chooses an artifact,
//! a creature, and a land from among the permanents controlled by the
//! player to their left. Destroy each permanent chosen this way." The
//! around-the-table choose-and-destroy is not expressible; the token is.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grenzo's Rebuttal");
    let _ogre = reg.interner_mut().intern("Ogre");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 4/4 red Ogre creature token. Starting with you, each player chooses an artifact, a creature, and a land from among the permanents controlled by the player to their left. Destroy each permanent chosen this way.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let ogre = reg.interner().lookup("Ogre").expect("Ogre interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    let token = TokenDefinition {
        name: ogre,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: around-the-table per-player choose-and-destroy not expressible.
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
