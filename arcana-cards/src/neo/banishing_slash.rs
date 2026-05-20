//! Banishing Slash — `{W}{W}` sorcery. "Destroy up to one target
//! artifact, enchantment, or tapped creature. Then if you control an
//! artifact and an enchantment, create a 2/2 white Samurai creature
//! token with vigilance."
//!
//! The conditional "if you control an artifact and an enchantment"
//! token rider has no catalog Effect (GAP'd); models only the destroy.

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
    let name = reg.interner_mut().intern("Banishing Slash");
    let _samurai = reg.interner_mut().intern("Samurai");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy up to one target artifact, enchantment, or tapped creature. Then if you control an artifact and an enchantment, create a 2/2 white Samurai creature token with vigilance.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types_any(TypeLine::ARTIFACT.into())
                        .with_types_any(TypeLine::ENCHANTMENT.into())
                        .with_types_any(TypeLine::CREATURE.into()),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: conditional token creation on "control artifact AND enchantment" not in catalog.
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    effects
}
