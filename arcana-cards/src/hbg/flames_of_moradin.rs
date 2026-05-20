//! Flames of Moradin — `{2}{R}{R}` sorcery. "Destroy up to three
//! target artifacts. Conjure a duplicate of each nontoken artifact
//! destroyed this way into your hand. The duplicates perpetually gain
//! ..."
//!
//! GAP: 'Conjure' / perpetual ability gain isn't modeled. The
//! destroy-up-to-three-artifacts portion is supported.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flames of Moradin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy up to three target artifacts. Conjure a duplicate of each nontoken artifact destroyed this way into your hand. The duplicates perpetually gain \"You may pay {R} rather than pay this spell's mana cost\" and \"At the beginning of your end step, sacrifice this artifact.\"".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
                ),
                count: TargetCount::UpTo(3),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in entry.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    // GAP: Conjure-duplicate-with-perpetual-rider on each destroyed nontoken artifact.
    effects
}
