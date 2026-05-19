//! Storm Herd — `{8}{W}{W}` sorcery. "Create X 1/1 white Pegasus creature
//! tokens with flying, where X is your life total."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Storm Herd");
    let _pegasus = reg.interner_mut().intern("Pegasus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create X 1/1 white Pegasus creature tokens with flying, where X is your life total.".into(),
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
    let x = script::life(state, entry.controller).max(0) as u32;
    let pegasus = reg.interner().lookup("Pegasus").expect("Pegasus interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);
    let token = TokenDefinition {
        name: pegasus,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    (0..x).map(|_| Effect::CreateToken { controller: entry.controller, token: token.clone() }).collect()
}
