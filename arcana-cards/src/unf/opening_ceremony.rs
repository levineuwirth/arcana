//! Opening Ceremony — `{4}{R}{R}` sorcery. "Add {W}{U}{B}{R}{G}{C}.
//! You may open a sealed Magic booster pack. Until end of turn, you
//! may cast spells from among the cards in that booster pack." The
//! booster-pack mechanic is silver-bordered and has no engine
//! representation — best-effort: add the WUBRGC mana.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Opening Ceremony");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Add {W}{U}{B}{R}{G}{C}. You may open a sealed Magic booster pack. Until end of turn, you may cast spells from among the cards in that booster pack.".into(),
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
    // GAP: 'open a sealed booster pack and cast from it' — silver-bordered/no engine support.
    vec![Effect::AddMana {
        player: entry.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, entry.source),
            ManaUnit::plain(ManaColor::Blue, entry.source),
            ManaUnit::plain(ManaColor::Black, entry.source),
            ManaUnit::plain(ManaColor::Red, entry.source),
            ManaUnit::plain(ManaColor::Green, entry.source),
            ManaUnit::plain(ManaColor::Colorless, entry.source),
        ],
    }]
}
