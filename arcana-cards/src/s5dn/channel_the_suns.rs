//! Channel the Suns — `{3}{G}` sorcery. "Add {W}{U}{B}{R}{G}."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Channel the Suns");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Add {W}{U}{B}{R}{G}.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: entry.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, entry.source),
            ManaUnit::plain(ManaColor::Blue, entry.source),
            ManaUnit::plain(ManaColor::Black, entry.source),
            ManaUnit::plain(ManaColor::Red, entry.source),
            ManaUnit::plain(ManaColor::Green, entry.source),
        ],
    }]
}
