//! Midnight Mayhem — `{2}{R}{W}` sorcery. "Create three 1/1 red
//! Gremlin creature tokens. Gremlins you control gain menace, lifelink,
//! and haste until end of turn."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Midnight Mayhem");
    let _gremlin = reg.interner_mut().intern("Gremlin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create three 1/1 red Gremlin creature tokens. Gremlins you control gain menace, lifelink, and haste until end of turn.".into(),
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
    let gremlin_name = reg.interner().lookup("Gremlin").expect("Gremlin interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gremlin_name);
    let token = TokenDefinition {
        name: gremlin_name,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ];
    let gremlin_filter = script::subtype_filter(reg, "Gremlin")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &gremlin_filter, entry.controller);
    for id in ids {
        for kw in [KeywordAbility::Menace, KeywordAbility::Lifelink, KeywordAbility::Haste] {
            effects.push(Effect::GrantKeyword {
                target: id,
                keyword: kw,
                duration: Duration::EndOfTurn,
            });
        }
    }
    effects
}
