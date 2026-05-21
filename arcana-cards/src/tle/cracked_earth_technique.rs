//! Cracked Earth Technique — `{4}{G}` Sorcery — Lesson. "Earthbend 3,
//! then earthbend 3. You gain 3 life."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cracked Earth Technique");
    let lesson = reg.interner_mut().intern("Lesson");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lesson);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Earthbend 3, then earthbend 3. You gain 3 life.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Earthbend (turn a land you control into a counter-bearing
    // creature with a return-on-death trigger) has no expressible Effect
    // variant; only the life gain is emitted.
    vec![Effect::GainLife { player: entry.controller, amount: 3 }]
}
