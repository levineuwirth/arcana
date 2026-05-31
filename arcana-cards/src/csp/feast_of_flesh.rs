//! Feast of Flesh — `{B}` sorcery. "Feast of Flesh deals X damage to
//! target creature and you gain X life, where X is 1 plus the number of
//! cards named Feast of Flesh in all graveyards."
//!
//! X is computed at resolution: 1 plus the count of cards named
//! "Feast of Flesh" across every player's graveyard. We sum
//! `script::graveyard_matching` over all players using a name-filtered
//! `ObjectFilter`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feast of Flesh");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Feast of Flesh deals X damage to target creature and you gain X life, where X is 1 plus the number of cards named Feast of Flesh in all graveyards.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };

    let nm = reg.interner().lookup("Feast of Flesh");
    let filter = ObjectFilter { name: nm, ..ObjectFilter::default() };
    let mut count: u32 = 0;
    for p in script::all_players(state) {
        count += script::graveyard_matching(state, &filter, p, entry.controller);
    }
    let x = 1 + count;

    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: x,
        },
        Effect::GainLife {
            player: entry.controller,
            amount: x,
        },
    ]
}
