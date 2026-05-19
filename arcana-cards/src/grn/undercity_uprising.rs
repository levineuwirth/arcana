//! Undercity Uprising — `{2}{B}{G}` sorcery, "Creatures you control gain
//! deathtouch until end of turn. Then target creature you control fights
//! target creature you don't control."
//!
//! GAP: "creatures you control gain deathtouch until end of turn" is a
//! ForEach GrantKeyword on all your creatures — expressible partially, but
//! "creatures you control" filter (controller == entry.controller) is not
//! in ObjectFilter. Best effort: fight only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undercity Uprising");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control gain deathtouch until end of turn. Then target creature you control fights target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
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
    // GAP: "creatures you control gain deathtouch until end of turn" —
    // ForEach with controller filter not expressible in ObjectFilter
    let Some(a_target) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(b_target) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(id_a) = a_target else { return Vec::new(); };
    let TargetChoice::Object(id_b) = b_target else { return Vec::new(); };
    vec![Effect::Fight { a: *id_a, b: *id_b }]
}
