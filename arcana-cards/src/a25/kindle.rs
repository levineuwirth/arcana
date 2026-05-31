//! Kindle — `{1}{R}` instant. "Kindle deals X damage to any target,
//! where X is 2 plus the number of cards named Kindle in all
//! graveyards." A burn spell whose damage scales with how many copies
//! of Kindle have already been cast into graveyards.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kindle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Kindle deals X damage to any target, where X is 2 plus the number of cards named Kindle in all graveyards.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
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
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    // X = 2 plus the number of cards named "Kindle" in all graveyards.
    let nm = reg.interner().lookup("Kindle");
    let filter = ObjectFilter { name: nm, ..ObjectFilter::default() };
    let mut named = 0u32;
    for p in script::all_players(state) {
        named += script::graveyard_matching(state, &filter, p, entry.controller);
    }
    let amount = 2 + named;
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount,
    }]
}
