//! Moogles' Valor — `{3}{W}{W}` instant, white. "For each creature you
//! control, create a 1/2 white Moogle creature token with lifelink. Then
//! creatures you control gain indestructible until end of turn."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moogles' Valor");
    let _moogle = reg.interner_mut().intern("Moogle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each creature you control, create a 1/2 white Moogle creature token with lifelink. Then creatures you control gain indestructible until end of turn.".into(),
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
    let moogle = reg.interner().lookup("Moogle").expect("Moogle interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moogle);
    let token = TokenDefinition {
        name: moogle,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects: Vec<Effect> = (0..n)
        .map(|_| Effect::CreateToken { controller: entry.controller, token: token.clone() })
        .collect();
    // Grant indestructible to all creatures you control
    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if !creature_ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::GrantKeyword {
                target: arcana_core::objects::NULL_OBJECT_ID,
                keyword: KeywordAbility::Indestructible,
                duration: Duration::EndOfTurn,
            }),
        });
    }
    effects
}
