//! Blood Money — `{5}{B}{B}` sorcery. "Destroy all creatures. For each
//! nontoken creature destroyed this way, you create a tapped Treasure
//! token."
//!
//! GAP: TokenDefinition has no "enters tapped" field, so the Treasures are
//! created untapped. The count is computed from the nontoken creatures
//! present at resolution.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blood Money");
    let _treasure = reg.interner_mut().intern("Treasure");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. For each nontoken creature destroyed this way, you create a tapped Treasure token.".into(),
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
    let all = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let nontoken =
        script::count_matching(state, &ObjectFilter::creature().nontoken(), entry.controller);
    let treasure = reg.interner().lookup("Treasure").expect("Treasure interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![Effect::ForEach {
        targets: all,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    // GAP: Treasures created untapped (no enters-tapped field).
    for _ in 0..nontoken {
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    effects
}
