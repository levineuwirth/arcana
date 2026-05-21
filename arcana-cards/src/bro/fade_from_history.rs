//! Fade from History — `{2}{G}{G}` sorcery. "Each player who controls
//! an artifact or enchantment creates a 2/2 green Bear creature
//! token. Then destroy all artifacts and enchantments."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fade from History");
    let _bear = reg.interner_mut().intern("Bear");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player who controls an artifact or enchantment creates a 2/2 green Bear creature token. Then destroy all artifacts and enchantments.".into(),
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
    let bear = reg
        .interner()
        .lookup("Bear")
        .expect("Bear interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    let token = TokenDefinition {
        name: bear,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };

    let mut out = Vec::new();
    for p in script::all_players(state) {
        let qualifying = script::count_matching(
            state,
            &ObjectFilter::permanent()
                .with_types_any(arcana_core::types::TypeLine(
                    TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
                ))
                .controlled_by(ControllerConstraint::You),
            p,
        );
        if qualifying > 0 {
            out.push(Effect::CreateToken {
                controller: p,
                token: token.clone(),
            });
        }
    }

    let _ = entry;
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types_any(
            arcana_core::types::TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT),
        ),
        entry.controller,
    );
    for id in ids {
        out.push(Effect::DestroyPermanent { target: id });
    }
    out
}
