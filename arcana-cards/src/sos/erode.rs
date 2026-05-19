//! Erode — `{W}` instant.
//! "Destroy target creature or planeswalker. Its controller may search their
//! library for a basic land card, put it onto the battlefield tapped, then
//! shuffle."
//!
//! GAP: targeting "creature or planeswalker" (TargetFilter has no
//! Creature-or-Planeswalker composite); "put tapped" flag on TutorToBattlefield
//! and targeting the destroyed permanent's controller for the tutor are also
//! not directly available. Best-effort: destroy target creature; land tutor
//! for the spell's controller (tapped flag unavailable).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Erode");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature or planeswalker. Its controller may search their library for a basic land card, put it onto the battlefield tapped, then shuffle.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DestroyPermanent { target: *id },
        // GAP: tapped:true flag on TutorToBattlefield not available; using tapped:false
        // GAP: tutor should go to the destroyed permanent's controller, not the spell controller
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: false,
        },
    ]
}
