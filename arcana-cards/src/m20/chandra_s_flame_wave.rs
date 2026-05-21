//! Chandra's Flame Wave — `{3}{R}{R}` sorcery. "Chandra's Flame Wave
//! deals 2 damage to target player and each creature that player
//! controls. Search your library and/or graveyard for a card named
//! Chandra, Flame's Fury, reveal it, and put it into your hand. If you
//! search your library this way, shuffle." Tutor by exact name from
//! library-or-graveyard isn't catalog-shaped; GAP that, emit the
//! damage shape.

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
    let name = reg.interner_mut().intern("Chandra's Flame Wave");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Chandra's Flame Wave deals 2 damage to target player and each creature that player controls. Search your library and/or graveyard for a card named Chandra, Flame's Fury, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let p = *p;
    let mut effects = vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(p),
        amount: 2,
    }];
    let ids = script::ids_matching(state, &ObjectFilter::creature(), p);
    for id in ids {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 2,
        });
    }
    // GAP: tutor-by-exact-name from library or graveyard.
    effects
}
