//! Exotic Pets — `{1}{W}{U}` instant. "Create two 1/1 blue Fish
//! creature tokens with \"This token can't be blocked.\" Then for each
//! kind of counter among creatures you control, put a counter of that
//! kind on either of those tokens."
//!
//! The two Fish tokens are created. Two clauses can't be expressed:
//! the token's intrinsic "can't be blocked" static (TokenDefinition has
//! no can't-be-blocked granting mechanism / continuous self-effect), and
//! the "for each kind of counter among creatures you control, put a
//! counter of that kind on a token" enumeration (no helper enumerates
//! the distinct counter KINDS present on permanents, nor lets the new
//! token ids be referenced at resolution).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exotic Pets");
    let _fish = reg.interner_mut().intern("Fish");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create two 1/1 blue Fish creature tokens with \"This token \
                   can't be blocked.\" Then for each kind of counter among \
                   creatures you control, put a counter of that kind on \
                   either of those tokens."
                .into(),
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
    let fish = reg
        .interner()
        .lookup("Fish")
        .expect("Fish interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    // GAP: token's intrinsic "This token can't be blocked." static cannot be
    // baked into a TokenDefinition.
    let token = TokenDefinition {
        name: fish,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "for each kind of counter among creatures you control, put a
    // counter of that kind on either of those tokens" — no helper enumerates
    // the distinct counter kinds on permanents, and the new token ids aren't
    // referenceable at resolution.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
