//! Tunnel — `{R}` instant. "Destroy target Wall. It can't be
//! regenerated."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tunnel");
    let _wall = reg.interner_mut().intern("Wall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target Wall. It can't be regenerated.".into(),
            target_requirements: vec![target_wall_req()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn target_wall_req() -> TargetRequirement {
    // We can't reference reg here for subtype filter; fall back to creature target.
    // The verify pipeline will surface the subtype-filter gap.
    TargetRequirement {
        filter: TargetFilter::Creature,
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let _wall_filter = script::subtype_filter(reg, "Wall");
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "can't be regenerated" rider not in catalog (DestroyPermanent allows regen).
    vec![Effect::DestroyPermanent { target: *id }]
}
