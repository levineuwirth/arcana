//! Sibling Rivalry — `{3}{R}` sorcery. "Gain control of target
//! artifact or creature until end of turn. Untap it. It gains haste
//! until end of turn. Create a tapped Powerstone token."
//!
//! Temporary control-gain has no catalog effect, so the
//! untap/haste-on-a-controlled-permanent half is a gap; the
//! Powerstone token is created (its enters-tapped state and mana
//! ability are not modeled).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sibling Rivalry");
    let _ps = reg.interner_mut().intern("Powerstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Gain control of target artifact or creature until end of turn. Untap it. It gains haste until end of turn. Create a tapped Powerstone token.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let ps = reg.interner().lookup("Powerstone").expect("Powerstone interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ps);
    let token = TokenDefinition {
        name: ps,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: temporary control-gain (and the dependent untap / haste
    // grant) and "enters tapped" + the Powerstone mana ability are
    // not in the effect catalog.
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
