//! Ceaseless Conflict — `{3}{W}{W}` sorcery, "Destroy all creatures. Then
//! create a 3/2 red and white Spirit creature token for each nontoken
//! creature you controlled that was destroyed this way."
//! GAP: counting nontoken creatures you controlled that were destroyed by
//! this spell (post-resolution graveyard delta not accessible).

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
    let name = reg.interner_mut().intern("Ceaseless Conflict");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. Then create a 3/2 red and white Spirit creature token for each nontoken creature you controlled that was destroyed this way.".into(),
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
    // Count your nontoken creatures before destruction for token creation
    let your_nontokens = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .nontoken()
            .controlled_by(arcana_core::targets::ControllerConstraint::You),
        entry.controller,
    );
    let token_count = your_nontokens.len() as u32;
    let all_creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![Effect::ForEach {
        targets: all_creatures,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }];
    // GAP: token count should be from nontoken creatures destroyed, not pre-resolution count;
    // using pre-resolution count as best approximation.
    for _ in 0..token_count {
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    effects
}
