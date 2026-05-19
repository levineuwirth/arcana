//! Pointed Discussion — `{2}{B}` sorcery.
//! "You draw two cards, lose 2 life, then create a Blood token."
//! GAP: Blood token has a specific activated ability ({1},{T},Discard,Sacrifice:Draw);
//! token abilities are not expressible in TokenDefinition.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pointed Discussion");
    let _blood = reg.interner_mut().intern("Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You draw two cards, lose 2 life, then create a Blood token.".into(),
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
    let blood = reg.interner().lookup("Blood")
        .expect("Blood interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(blood);
    let token = TokenDefinition {
        name: blood,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        // GAP: activated ability {1},{T},Discard a card,Sacrifice: Draw a card
        abilities: vec![],
    };
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::LoseLife { player: entry.controller, amount: 2 },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
