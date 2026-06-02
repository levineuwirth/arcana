//! Spore Burst — `{3}{G}` sorcery. "Domain — Create a 1/1 green
//! Saproling creature token for each basic land type among lands you
//! control."
//!
//! Domain is dynamic: the count is the number of distinct basic land
//! types (Plains/Island/Swamp/Mountain/Forest) present among lands you
//! control. We compute it at resolution by checking, for each of the
//! five basic land subtypes, whether you control at least one land with
//! that subtype, then create that many Saproling tokens.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spore Burst");
    let _saproling = reg.interner_mut().intern("Saproling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Create a 1/1 green Saproling creature token for each basic land type among lands you control.".into(),
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
    let basic_types = ["Plains", "Island", "Swamp", "Mountain", "Forest"];
    let mut domain: u32 = 0;
    for ty in basic_types {
        let filter = script::subtype_filter(reg, ty)
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, entry.controller) > 0 {
            domain += 1;
        }
    }

    let saproling = reg
        .interner()
        .lookup("Saproling")
        .expect("Saproling interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saproling);
    let token = TokenDefinition {
        name: saproling,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };

    (0..domain)
        .map(|_| Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        })
        .collect()
}
