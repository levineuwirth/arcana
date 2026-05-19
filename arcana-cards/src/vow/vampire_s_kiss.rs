//! Vampire's Kiss — `{1}{B}` sorcery. "Target player loses 2 life and you
//! gain 2 life. Create two Blood tokens."
//!
//! # GAP
//! Blood tokens require an activated ability ("{1}, {T}, Discard a card,
//! Sacrifice this token: Draw a card") which is not expressible via the
//! `TokenDefinition` API (no `activated_abilities` field). The two
//! `Effect::CreateToken` calls below create bare artifact tokens without
//! the Blood ability.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vampire's Kiss");
    let _blood = reg.interner_mut().intern("Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player loses 2 life and you gain 2 life. Create two Blood tokens.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let blood = reg.interner().lookup("Blood").expect("Blood interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(blood);
    // GAP: TokenDefinition has no activated_abilities field; Blood token ability cannot be expressed
    let token = TokenDefinition {
        name: blood,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::LoseLife { player: *p, amount: 2 },
        Effect::GainLife { player: entry.controller, amount: 2 },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
