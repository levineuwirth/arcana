//! Covetous Elegy — `{4}{W}{B}` sorcery. "Each player chooses up to
//! two creatures they control, then sacrifices the rest. Then you
//! create a tapped Treasure token for each creature your opponents
//! control."
//!
//! The first clause (each player keeps up to two creatures and
//! sacrifices the rest) is a player-directed partial-selection
//! sacrifice with no matching engine primitive — `Effect::Sacrifice`
//! takes a fixed count, not a "keep up to N, sac the rest" choice —
//! so it is GAP'd. The second clause is expressible: the count of
//! creatures your opponents control is computed dynamically and that
//! many Treasure tokens are minted (with a fidelity gap on "tapped",
//! which `CreateCommodityToken` does not model).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Covetous Elegy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player chooses up to two creatures they control, then sacrifices the rest. Then you create a tapped Treasure token for each creature your opponents control.".into(),
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
    // GAP: "each player chooses up to two creatures they control, then
    // sacrifices the rest" — partial-selection sacrifice (keep up to N,
    // sac remainder) has no engine primitive; Effect::Sacrifice is a
    // fixed-count engine-chosen sacrifice.
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    // GAP: the Treasure tokens should enter tapped; CreateCommodityToken
    // does not model the tapped rider.
    vec![Effect::CreateCommodityToken {
        controller: entry.controller,
        kind: CommodityToken::Treasure,
        count: n,
    }]
}
