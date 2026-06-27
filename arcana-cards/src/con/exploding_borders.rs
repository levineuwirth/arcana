//! Exploding Borders — `{2}{R}{G}` sorcery. "Domain — Search your
//! library for a basic land card, put that card onto the battlefield
//! tapped, then shuffle. Exploding Borders deals X damage to target
//! player or planeswalker, where X is the number of basic land types
//! among lands you control."
//!
//! The ramp half is expressible: tutor a basic land to the
//! battlefield tapped (shuffle is automatic). The damage half scales
//! with Domain (`script::domain` — number of basic land types among
//! lands you control) and is dealt to the chosen target.
//! GAP (fidelity): "target player or planeswalker" is approximated with
//! the any-target requirement (creatures are also selectable — there is
//! no player-or-planeswalker-only TargetFilter). X is snapshotted before
//! the searched land enters, so a newly-tutored basic land type is not
//! counted toward this resolution's damage.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exploding Borders");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle. Exploding Borders deals X damage to target player or planeswalker, where X is the number of basic land types among lands you control.".into(),
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
    let mut effects = vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(arcana_core::types::SupertypeSet::new().with(arcana_core::types::SupertypeSet::BASIC)),
        tapped: true,
    }];
    // X = domain (number of basic land types among lands you control).
    let x = script::domain(state, entry.controller, reg);
    // GAP: "target player or planeswalker" — any_target also admits
    // creatures (no player-or-planeswalker-only filter).
    if let Some(target) = entry.targets.targets.first() {
        let dt = match target {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
                Some(DamageTarget::Object(*id))
            }
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
                Some(DamageTarget::Player(*p))
            }
            _ => None,
        };
        if let Some(dt) = dt {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: dt,
                amount: x,
            });
        }
    }
    effects
}
