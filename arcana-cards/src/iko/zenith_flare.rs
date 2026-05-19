//! Zenith Flare — `{2}{R}{W}` instant. "Zenith Flare deals X damage to
//! any target and you gain X life, where X is the number of cards with
//! a cycling ability in your graveyard."
//!
//! GAP: no graveyard filter for "has cycling ability" in
//! graveyard_matching. Best-effort: use graveyard_size as approximation
//! (over-counts). Using Vec::new() since graveyard_matching with a
//! cycling-keyword filter is not available.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zenith Flare");
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
                text: "Zenith Flare deals X damage to any target and you gain X life, where X is the number of cards with a cycling ability in your graveyard.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: count only cards with cycling keyword in graveyard; using
    // graveyard_size as over-broad approximation.
    let x = script::graveyard_size(state, entry.controller);
    if x == 0 {
        return Vec::new();
    }
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: x,
        },
        Effect::GainLife { player: entry.controller, amount: x },
    ]
}
