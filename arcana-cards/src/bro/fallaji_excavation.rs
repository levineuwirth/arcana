//! Fallaji Excavation — `{3}{G}{G}` sorcery.
//! "Create three tapped Powerstone tokens. You gain 3 life."
//
// GAP: Powerstone token is an artifact with a special mana ability
//      ("{T}: Add {C}. This mana can't be spent to cast a nonartifact
//      spell."). The TokenDefinition has no field for activated abilities
//      beyond `abilities: vec![]`, so the Powerstone's tap ability is lost.
//      Best effort: create three artifact tokens + gain 3 life.
// GAP: tokens enter tapped (no `tapped` field on TokenDefinition).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fallaji Excavation");
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create three tapped Powerstone tokens. You gain 3 life.".into(),
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
    use arcana_core::types::SubtypeSet;
    let powerstone = reg.interner().lookup("Powerstone").expect("Powerstone interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(powerstone);
    // GAP: Powerstone tap ability; GAP: tokens enter tapped
    let token = TokenDefinition {
        name: powerstone,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
        Effect::GainLife { player: entry.controller, amount: 3 },
    ]
}
