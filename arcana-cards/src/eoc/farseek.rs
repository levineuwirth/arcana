//! Farseek — `{1}{G}` sorcery. "Search your library for a Plains,
//! Island, Swamp, or Mountain card, put it onto the battlefield
//! tapped, then shuffle." The catalog's `subtype_filter` only takes
//! one name — best effort: tutor a Plains. (The other three basic
//! types are a GAP for the multi-subtype filter.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Farseek");
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a Plains, Island, Swamp, or Mountain card, put it onto the battlefield tapped, then shuffle.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: subtype_filter takes a single subtype; the catalog has no
    // disjunction across Plains/Island/Swamp/Mountain. Tutor for Plains
    // as a best-effort representative.
    let filter = script::subtype_filter(reg, "Plains");
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter,
        tapped: true,
    }]
}
