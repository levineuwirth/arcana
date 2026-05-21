//! Heartwarming Redemption — `{2}{R}{W}` instant. "Discard all the cards in
//! your hand, then draw that many cards plus one. You gain life equal to the
//! number of cards in your hand."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heartwarming Redemption");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard all the cards in your hand, then draw that many cards plus one. You gain life equal to the number of cards in your hand.".into(),
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
    let h = script::hand_size(state, entry.controller);
    let mut effects: Vec<Effect> = Vec::new();
    if h > 0 {
        effects.push(Effect::Discard {
            player: entry.controller,
            count: h,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects.push(Effect::DrawCards {
        player: entry.controller,
        count: h + 1,
    });
    // Hand size after redraw equals h+1 (h discarded → 0 → draw h+1). Use that.
    effects.push(Effect::GainLife {
        player: entry.controller,
        amount: h + 1,
    });
    effects
}
