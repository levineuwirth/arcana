//! Gaea's Might — `{G}` instant. "Domain — Target creature gets +1/+1
//! until end of turn for each basic land type among lands you control."
//!
//! Domain is a dynamic count (CR 700.10): the number of the five basic
//! land types (Plains, Island, Swamp, Mountain, Forest) present among
//! the lands the controller controls. The resolver counts how many of
//! those subtypes appear on at least one controlled land and pumps the
//! target by that amount.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gaea's Might");
    // Intern the five basic land subtypes for the Domain count.
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Target creature gets +1/+1 until end of turn for each basic land type among lands you control.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };

    let mut domain: i32 = 0;
    for ty in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let Some(sym) = reg.interner().lookup(ty) else { continue; };
        let filter = ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You)
            .with_subtypes_any(vec![sym]);
        if script::count_matching(state, &filter, entry.controller) > 0 {
            domain += 1;
        }
    }

    vec![Effect::Pump {
        target: *id,
        power: domain,
        toughness: domain,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
