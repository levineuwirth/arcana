//! Tribal Flames — `{1}{R}` sorcery. "Domain — Tribal Flames deals X
//! damage to any target, where X is the number of basic land types
//! among lands you control." Domain X = count distinct basic land
//! subtypes among lands you control (max 5). The script surface has
//! `subtype_filter` and `count_matching` but no 'distinct subtypes
//! among lands' primitive. We approximate by checking each of the
//! five basic subtypes.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tribal Flames");
    // Pre-intern basic land subtype names so subtype_filter can look them up.
    let _ = reg.interner_mut().intern("Plains");
    let _ = reg.interner_mut().intern("Island");
    let _ = reg.interner_mut().intern("Swamp");
    let _ = reg.interner_mut().intern("Mountain");
    let _ = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Tribal Flames deals X damage to any target, where X is the number of basic land types among lands you control.".into(),
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
    let mut x: u32 = 0;
    for ty in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let f = script::subtype_filter(reg, ty).controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &f, entry.controller) > 0 {
            x += 1;
        }
    }
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: x,
    }]
}
