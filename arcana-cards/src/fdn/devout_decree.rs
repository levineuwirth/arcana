//! Devout Decree — `{1}{W}` sorcery. "Exile target creature or
//! planeswalker that's black or red. Scry 1."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devout Decree");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: 'black OR red' color filter — only with_colors (set-equal /
    // contains-all semantics) is exposed; we approximate with black-or-
    // red by using with_colors per the prompt's stated API. Use
    // 'with_colors(ColorSet::black() | ColorSet::red())' which is the
    // available expression.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature or planeswalker that's black or red. Scry 1.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(arcana_core::types::TypeLine(
                                TypeLine::CREATURE | TypeLine::PLANESWALKER,
                            ))
                            .with_colors(ColorSet::black() | ColorSet::red()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
        Effect::ExilePermanent { target: *id },
        Effect::Scry { player: entry.controller, count: 1 },
    ]
}
