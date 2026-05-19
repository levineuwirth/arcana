//! Faerie Slumber Party — `{4}{U}{U}` sorcery. "Return all creatures to their
//! owners' hands. For each opponent who controlled a creature returned this
//! way, you create two 1/1 blue Faerie creature tokens with flying and 'This
//! token can block only creatures with flying.'"
//!
//! # GAP: BlockRestrictionOnToken — no way to express "can block only creatures
//! with flying" on a token. The token is created with Flying only.
//! GAP: OpponentCreatureReturnCount — no script helper for counting distinct
//! opponents who controlled a returned creature. Two tokens are always created
//! (one opponent assumed); correct count requires opponent enumeration.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Faerie Slumber Party");
    let _faerie = reg.interner_mut().intern("Faerie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all creatures to their owners' hands. For each opponent who controlled a creature returned this way, you create two 1/1 blue Faerie creature tokens with flying.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let faerie = reg.interner().lookup("Faerie").expect("Faerie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    // GAP: BlockRestrictionOnToken — "can block only creatures with flying" not representable.
    // GAP: OpponentCreatureReturnCount — always creating 2 tokens; accurate count needs player enumeration.
    let token = TokenDefinition {
        name: faerie,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    let mut effects: Vec<Effect> = vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::ReturnToHand { target: arcana_core::objects::NULL_OBJECT_ID }),
        },
    ];
    effects.push(Effect::CreateToken { controller: entry.controller, token: token.clone() });
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
