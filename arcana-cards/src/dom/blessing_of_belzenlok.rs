//! Blessing of Belzenlok — `{B}` instant. "Target creature gets +2/+1 until
//! end of turn. If it's legendary, it also gains lifelink until end of turn."
//!
//! GAP: Conditional effect based on a permanent's legendary supertype at
//! resolve time is not expressible with Effect::Conditional (the catalog shows
//! a Conditional variant but its `condition` type is not demonstrated). Best-effort:
//! render the Pump unconditionally; the lifelink-if-legendary conditional is not expressible.

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
    let name = reg.interner_mut().intern("Blessing of Belzenlok");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +2/+1 until end of turn. If it's legendary, it also gains lifelink until end of turn.".into(),
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
    // GAP: Effect::Conditional `condition` type not demonstrated; cannot branch on legendary supertype.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
