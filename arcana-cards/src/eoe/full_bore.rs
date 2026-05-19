//! Full Bore — `{R}` instant, "Target creature you control gets +3/+2
//! until end of turn. If that creature was cast for its warp cost, it
//! also gains trample and haste until end of turn."
//!
//! GAP: warp-cast conditional for trample+haste not expressible; expressed
//! as flat +3/+2 pump.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Full Bore");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +3/+2 until end of turn. If that creature was cast for its warp cost, it also gains trample and haste until end of turn.".into(),
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
    // GAP: warp-cast conditional for Trample+Haste grant
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
