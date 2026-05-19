//! Banishing Slash — `{W}{W}` sorcery, "Destroy up to one target artifact,
//! enchantment, or tapped creature. If you controlled a Samurai or Warrior as
//! you cast this spell, create a 2/2 white Samurai creature token with
//! vigilance."
//!
//! # GAP
//! No "tapped creature" ObjectFilter predicate; no conditional token creation
//! based on "controlled a Samurai or Warrior as you cast"; falling back to
//! destroy any creature/artifact/enchantment.

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
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy up to one target artifact, enchantment, or tapped creature. If you controlled a Samurai or Warrior as you cast this spell, create a 2/2 white Samurai creature token with vigilance.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT | TypeLine::CREATURE),
                        ),
                    ),
                    count: TargetCount::UpTo(1),
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
    // GAP: no "tapped creature" filter; no conditional Samurai token creation
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
