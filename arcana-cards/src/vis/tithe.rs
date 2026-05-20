//! Tithe — `{W}` instant. "Search your library for a Plains card. If
//! target opponent controls more lands than you, you may search your
//! library for an additional Plains card. Reveal those cards, put
//! them into your hand, then shuffle." The conditional second search
//! keyed on a land-count comparison is not expressible; a land tutor
//! to hand is the closest faithful primitive (subtype-restricting the
//! search to Plains is not in the ObjectFilter surface).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tithe");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for a Plains card. If target opponent controls more lands than you, you may search your library for an additional Plains card. Reveal those cards, put them into your hand, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: conditional additional Plains search keyed on opponent's land
    // count vs yours is not expressible; Plains-subtype restriction is
    // also outside the ObjectFilter surface — base land tutor only.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        reveal: true,
    }]
}
