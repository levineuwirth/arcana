//! Tyranid Invasion — `{3}{G}` sorcery. "Create a number of 3/3 green Tyranid
//! Warrior creature tokens with trample equal to the number of opponents you have."
//!
//! GAP: "number of opponents" is not available via script:: helpers (no
//! opponent_count function). In a 2-player game this is always 1; emitting 1
//! token as a best-effort skeleton.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyranid Invasion");
    let _tyranid = reg.interner_mut().intern("Tyranid");
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
                text: "Create a number of 3/3 green Tyranid Warrior creature tokens with trample equal to the number of opponents you have.".into(),
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
    // GAP: number of opponents (no script::opponent_count helper); hard-coding 1 for 2-player
    let tyranid = reg.interner().lookup("Tyranid").expect("Tyranid interned during register()");
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: tyranid,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
