//! Mogg Infestation — `{3}{R}{R}` sorcery. "Destroy all creatures
//! target player controls. For each creature that died this way,
//! that player creates two 1/1 red Goblin creature tokens."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mogg Infestation");
    let _gob = reg.interner_mut().intern("Goblin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures target player controls. For each creature that died this way, that player creates two 1/1 red Goblin creature tokens.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let owned = script::ids_matching(state, &ObjectFilter::creature(), *p);
    let n = owned.len();
    let gob = reg.interner().lookup("Goblin").expect("Goblin interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gob);
    let token = TokenDefinition {
        name: gob,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![Effect::ForEach {
        targets: owned,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }];
    // Two tokens per creature destroyed this way.
    for _ in 0..(n * 2) {
        effects.push(Effect::CreateToken {
            controller: *p,
            token: token.clone(),
        });
    }
    effects
}
