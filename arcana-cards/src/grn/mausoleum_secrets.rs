//! Mausoleum Secrets — `{1}{B}` instant. "Undergrowth — Search your
//! library for a black card with mana value less than or equal to the
//! number of creature cards in your graveyard, reveal it, put it into
//! your hand, then shuffle."
//!
//! The mana-value cap is dynamic (the number of creature cards in your
//! graveyard), so the count is computed at resolution with
//! `script::graveyard_matching` and fed into the tutor filter's max CMC.
//! Undergrowth has no separate keyword variant — its only effect is the
//! graveyard-count scaling, which is captured here.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mausoleum Secrets");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Undergrowth — Search your library for a black card with mana value less than or equal to the number of creature cards in your graveyard, reveal it, put it into your hand, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creatures_in_yard = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
        entry.controller,
    );
    let filter = ObjectFilter::new()
        .with_colors(ColorSet::black())
        .with_max_cmc(creatures_in_yard);
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter,
        reveal: true,
    }]
}
