//! Last-Ditch Effort — `{R}` instant. "Sacrifice any number of
//! creatures. Last-Ditch Effort deals that much damage to any target."
//! The variable sacrifice is modeled via a player-chosen any-number
//! pick over your creatures.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Last-Ditch Effort");
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
                text: "Sacrifice any number of creatures. Last-Ditch Effort deals that much damage to any target.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
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
    // The "sacrifice any number of creatures" is a variable player-chosen
    // pick over your creatures.
    // GAP: "deals that much damage to any target" — there is no way to read
    // back the number of creatures sacrificed by ChooseAnyNumberFromZone
    // and feed that count into a DealDamage amount. The sacrifice is
    // modeled; the dependent damage rider is dropped (no fixed stand-in).
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: entry.controller,
        zone: Zone::Battlefield,
        filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        action: PickAction::Sacrifice,
    }]
}
