//! Thunder of Hooves — `{3}{R}` sorcery. "Thunder of Hooves deals X
//! damage to each creature without flying and each player, where X is
//! the number of Beasts on the battlefield."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thunder of Hooves");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Thunder of Hooves deals X damage to each creature without flying and each player, where X is the number of Beasts on the battlefield.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Beast"),
        entry.controller,
    );
    // GAP: "each creature without flying" — no "lacks flying" filter,
    // so the creature sweep is omitted (damaging all creatures would
    // be materially wrong); the each-player damage is modeled.
    let mut out = Vec::new();
    for p in script::all_players(state) {
        out.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: x,
        });
    }
    out
}
