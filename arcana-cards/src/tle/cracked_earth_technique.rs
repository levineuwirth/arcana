//! Cracked Earth Technique — `{4}{G}` sorcery (Lesson). "Earthbend
//! 3, then earthbend 3. You gain 3 life." The earthbend mechanic
//! (turn a land into a creature with counters and a return-on-death
//! rider) is not modeled; we emit the "you gain 3 life" component.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cracked Earth Technique");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
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
    // GAP: earthbend (land becomes a creature with +1/+1 counters and
    // a return-on-death rider) is not modeled. Life gain emitted.
    vec![Effect::GainLife { player: entry.controller, amount: 3 }]
}
