//! Revenge of the Drowned — `{3}{U}` instant. "Target creature's
//! owner puts it on their choice of the top or bottom of their
//! library. You create a 2/2 black Zombie creature token with
//! decayed."
//!
//! The owner's top-or-bottom choice is not expressible; we put the
//! target on top of its owner's library. The decayed keyword is not
//! in the catalog keyword surface, so the token is created without
//! it.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Revenge of the Drowned");
    let _z = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature's owner puts it on their choice of the top or bottom of their library. You create a 2/2 black Zombie creature token with decayed.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let z = reg.interner().lookup("Zombie").expect("Zombie interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(z);
    let token = TokenDefinition {
        name: z,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: owner's top-or-bottom choice not expressible (top used);
    // decayed keyword not in catalog surface.
    vec![
        Effect::PutOnTopOfLibrary { target: *id },
        Effect::CreateToken {
            controller: entry.controller,
            token,
        },
    ]
}
