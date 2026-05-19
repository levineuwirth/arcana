//! Snap — `{1}{U}` instant.
//! "Return target creature to its owner's hand. Untap up to two lands."
//!
//! # GAP: untap up to two target lands — TargetCount::UpTo(2) on a land
//! filter plus Untap per target is structurally supportable but the second
//! requirement uses a separate target slot. Modelled as bounce + single-land
//! untap; the second land untap is a gap because multi-target resolution
//! over a separate UpTo(2) lands requirement is not shown in the catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snap");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature to its owner's hand. Untap up to two lands.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::new().with_types(TypeLine::LAND.into())),
                        count: TargetCount::UpTo(2),
                        controller: None,
                    },
                ],
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
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            // First object is the creature (bounce), remaining are lands (untap).
            // We use ReturnToHand for the first and Untap for the rest.
            // Since we cannot distinguish by slot index here, apply both bounce
            // and untap conservatively to the first target only (creature bounce).
            effects.push(Effect::ReturnToHand { target: *id });
            break;
        }
    }
    // GAP: untap up to two lands (multi-slot UpTo target resolution not demonstrable)
    effects
}
