//! Tend the Sprigs — `{2}{G}` sorcery. "Search your library for a basic
//! land card, put it onto the battlefield tapped, then shuffle. Then if
//! you control seven or more lands and/or Treefolk, create a 3/4 green
//! Treefolk creature token with reach."

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
    let name = reg.interner_mut().intern("Tend the Sprigs");
    let _treefolk = reg.interner_mut().intern("Treefolk");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for a basic land card, put it onto the \
                   battlefield tapped, then shuffle. Then if you control seven \
                   or more lands and/or Treefolk, create a 3/4 green Treefolk \
                   creature token with reach."
                .into(),
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
    let treefolk = reg
        .interner()
        .lookup("Treefolk")
        .expect("Treefolk interned during register()");
    let mut effects = vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        tapped: true,
    }];
    let lands = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(arcana_core::targets::ControllerConstraint::You),
        entry.controller,
    );
    if lands >= 7 {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(treefolk);
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: treefolk,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(4)),
                keywords: vec![KeywordAbility::Reach],
                abilities: vec![],
            },
        });
    }
    effects
}
