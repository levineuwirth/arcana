//! Necrotic Hex — `{6}{B}` sorcery. "Each player sacrifices six
//! creatures of their choice. You create six tapped 2/2 black Zombie
//! creature tokens."
//!
//! TokenDefinition has no tapped flag; the tokens enter untapped
//! (the "tapped" rider is a minor GAP). Per-player sacrifice and the
//! six tokens are emitted.

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
    let name = reg.interner_mut().intern("Necrotic Hex");
    let _z = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player sacrifices six creatures of their choice. You create six tapped 2/2 black Zombie creature tokens.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let z = reg
        .interner()
        .lookup("Zombie")
        .expect("Zombie interned during register()");
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
    let mut out = Vec::new();
    for p in script::all_players(state) {
        out.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 6,
        });
    }
    // GAP: TokenDefinition has no tapped flag; tokens enter untapped.
    for _ in 0..6 {
        out.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    out
}
