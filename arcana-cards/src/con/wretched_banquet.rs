//! Wretched Banquet — `{B}` sorcery. "Destroy target creature if it has
//! the least power or is tied for least power among creatures on the
//! battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wretched Banquet");
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
                text: "Destroy target creature if it has the least power or is tied for least power among creatures on the battlefield.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let tgt_power = script::power_of(state, *id);
    let all = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let least = all
        .iter()
        .map(|c| script::power_of(state, *c))
        .min()
        .unwrap_or(tgt_power);
    if tgt_power <= least {
        vec![Effect::DestroyPermanent { target: *id }]
    } else {
        Vec::new()
    }
}
