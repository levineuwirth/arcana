//! Syphon Flesh — `{4}{B}` sorcery. "Each other player sacrifices a
//! creature. You create a 2/2 black Zombie creature token for each
//! creature sacrificed this way."
//!
//! Dynamic token count = number of opponents (1 sacrifice per opponent).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syphon Flesh");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each other player sacrifices a creature. You create a 2/2 black Zombie creature token for each creature sacrificed this way.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    let opps = script::opponents(state, entry.controller);
    let mut effects: Vec<Effect> = opps
        .iter()
        .map(|p| Effect::Sacrifice {
            player: *p,
            filter: ObjectFilter::creature(),
            count: 1,
        })
        .collect();
    for _ in 0..opps.len() {
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    effects
}
