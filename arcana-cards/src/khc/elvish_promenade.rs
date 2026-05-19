//! Elvish Promenade — `{3}{G}` Kindred Sorcery — Elf.
//! "Create a 1/1 green Elf Warrior creature token for each Elf you control."
//! Type line is Kindred Sorcery; engine only supports SORCERY, so using SORCERY.

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
    let name = reg.interner_mut().intern("Elvish Promenade");
    let _elf = reg.interner_mut().intern("Elf");
    let _warrior = reg.interner_mut().intern("Warrior");
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
                text: "Create a 1/1 green Elf Warrior creature token for each Elf you control.".into(),
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
    let elf_filter = script::subtype_filter(reg, "Elf");
    let count = script::count_matching(state, &elf_filter, entry.controller);

    let elf = reg.interner().lookup("Elf").expect("Elf interned during register()");
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let token = TokenDefinition {
        name: elf,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };

    (0..count)
        .map(|_| Effect::CreateToken { controller: entry.controller, token: token.clone() })
        .collect()
}
