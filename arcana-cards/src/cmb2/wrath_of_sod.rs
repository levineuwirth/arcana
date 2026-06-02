//! Wrath of Sod — `{2}{G}{W}` sorcery. "Put a manabond counter on all
//! creatures. (They lose all other abilities and become lands with
//! "{T}: Add one mana of this card's color.")"
//!
//! The literal effect — placing a "manabond" counter on every creature
//! on the battlefield — is expressible as a `ForEach` `AddCounters` with
//! a `Named` counter. The parenthetical describes the static rules the
//! counter confers (lose abilities / become a mana-land); that
//! counter-driven type-change static is not modeled, so it is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrath of Sod");
    let _manabond = reg.interner_mut().intern("manabond");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a manabond counter on all creatures. (They lose all other abilities and become lands with \"{T}: Add one mana of this card's color.\")".into(),
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
    let manabond = reg.interner().lookup("manabond")
        .expect("manabond interned during register()");
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::Named(manabond),
            count: 1,
        }),
    }]
    // GAP: the manabond counter's static effect (creatures lose all
    // other abilities and become lands with "{T}: Add one mana of this
    // card's color") is a counter-driven type-change/ability-strip
    // static that is not modeled.
}
