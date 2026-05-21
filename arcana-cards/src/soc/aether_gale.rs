//! Aether Gale — `{3}{U}{U}` sorcery. "Return six target nonland
//! permanents to their owners' hands."

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
    let name = reg.interner_mut().intern("Aether Gale");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let nonland = || ObjectFilter::permanent().without_types(TypeLine::LAND.into());
    let mut reqs: Vec<TargetRequirement> = Vec::with_capacity(6);
    for _ in 0..6 {
        reqs.push(TargetRequirement {
            filter: TargetFilter::Permanent(nonland()),
            count: TargetCount::Exactly(1),
            controller: None,
        });
    }
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return six target nonland permanents to their owners' hands.".into(),
            target_requirements: reqs,
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
    let mut out: Vec<Effect> = Vec::new();
    for t in entry.targets.targets.iter().take(6) {
        if let TargetChoice::Object(id) = t {
            out.push(Effect::ReturnToHand { target: *id });
        }
    }
    out
}
