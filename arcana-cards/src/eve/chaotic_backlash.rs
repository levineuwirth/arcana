//! Chaotic Backlash — `{4}{R}` instant. Deals damage to target player
//! equal to twice the number of white and/or blue permanents they
//! control.

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
    let name = reg.interner_mut().intern("Chaotic Backlash");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Chaotic Backlash deals damage to target player equal to twice the number of white and/or blue permanents they control.".into(),
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
    // GAP: "white and/or blue" union — approximate by counting whites + blues
    // (double-counts WU permanents; engine has no union-color filter).
    let whites = script::count_matching(
        state,
        &ObjectFilter::permanent().with_colors(ColorSet::white()),
        p,
    );
    let blues = script::count_matching(
        state,
        &ObjectFilter::permanent().with_colors(ColorSet::blue()),
        p,
    );
    let amount = (whites + blues) * 2;
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(p),
        amount,
    }]
}
