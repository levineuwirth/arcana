//! Meeting of the Five — `{3}{W}{U}{B}{R}{G}` sorcery. "Exile the
//! top ten cards of your library. You may cast spells with exactly
//! three colors from among them this turn. Add {W}{W}{U}{U}{B}{B}{R}{R}{G}{G}."
//!
//! GAP: 'exile top ten and play from exile' impulse-draw, plus
//! mana-restriction tagging, aren't modeled — emit only the mana
//! generation (a ten-pip rainbow ritual).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meeting of the Five");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile the top ten cards of your library. You may cast spells with exactly three colors from among them this turn. Add {W}{W}{U}{U}{B}{B}{R}{R}{G}{G}. Spend this mana only to cast spells with exactly three colors.".into(),
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
    let mut mana = Vec::new();
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        mana.push(ManaUnit::plain(color, entry.source));
        mana.push(ManaUnit::plain(color, entry.source));
    }
    // GAP: 'exile top ten / play during turn' impulse window and
    // mana-spend restriction not modeled.
    vec![Effect::AddMana { player: entry.controller, mana }]
}
