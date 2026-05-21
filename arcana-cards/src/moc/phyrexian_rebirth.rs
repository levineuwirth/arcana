//! Phyrexian Rebirth — `{4}{W}{W}` sorcery. "Destroy all creatures,
//! then create an X/X colorless Phyrexian Horror artifact creature
//! token, where X is the number of creatures destroyed this way."
//! X = creatures-on-battlefield at resolution (which will all be
//! destroyed). We compute X BEFORE the wipe and use it for the token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Rebirth");
    let _phyrexian = reg.interner_mut().intern("Phyrexian");
    let _horror = reg.interner_mut().intern("Horror");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures, then create an X/X colorless Phyrexian Horror artifact creature token, where X is the number of creatures destroyed this way.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let x = ids.len() as i32;
    let phyr = reg.interner().lookup("Phyrexian").expect("Phyrexian interned during register()");
    let horror = reg.interner().lookup("Horror").expect("Horror interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyr);
    subtypes.0.insert(horror);
    let token = TokenDefinition {
        name: horror,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(x)),
        toughness: Some(PtValue::Fixed(x)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
