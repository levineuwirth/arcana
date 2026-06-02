//! Entrapment Maneuver — `{3}{W}` instant. "Target player sacrifices an
//! attacking creature of their choice. You create X 1/1 white Soldier
//! creature tokens, where X is that creature's toughness."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Entrapment Maneuver");
    let _soldier = reg.interner_mut().intern("Soldier");
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
                text: "Target player sacrifices an attacking creature of their choice. You create X 1/1 white Soldier creature tokens, where X is that creature's toughness.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // The sacrifice itself is expressible; the "attacking" restriction on
    // the filter is not, so this sacrifices any creature of their choice.
    // GAP: create X 1/1 white Soldier tokens where X is the SACRIFICED
    // creature's toughness — the resolver cannot reference the just-
    // sacrificed creature's id, so the token count cannot be computed.
    vec![Effect::Sacrifice {
        player: *p,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
