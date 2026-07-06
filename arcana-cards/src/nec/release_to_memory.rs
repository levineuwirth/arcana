//! Release to Memory — `{3}{W}` instant, "Exile target opponent's graveyard.
//! For each creature card exiled this way, create a 1/1 colorless Spirit
//! creature token."
//! "Exile a whole graveyard and count creature cards" is not expressible as
//! a single Effect. Best effort: use `script::graveyard_matching` to count,
//! then create that many tokens; no bulk graveyard-exile effect exists.
//!
//! # GAP: exile-entire-graveyard effect (no Effect variant to exile all cards in a player's graveyard)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Release to Memory");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target opponent's graveyard. For each creature card exiled this way, create a 1/1 colorless Spirit creature token.".into(),
                target_requirements: vec![TargetRequirement::target_opponent()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let creature_filter = ObjectFilter::creature();
    let n = script::graveyard_matching(state, &creature_filter, *p, entry.controller);
    // GAP: exile-entire-graveyard effect (no Effect variant to exile all cards in a player's graveyard)
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n).map(|_| Effect::CreateToken {
        controller: entry.controller,
        token: token.clone(),
    }).collect()
}
