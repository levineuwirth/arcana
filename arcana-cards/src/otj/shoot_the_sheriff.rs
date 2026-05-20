//! Shoot the Sheriff — `{1}{B}` instant. "Destroy target non-outlaw
//! creature." Outlaws are Assassin/Mercenary/Pirate/Rogue/Warlock — there's no
//! "without any of these subtypes" filter; closest expressible target is plain
//! creature with a doc note. GAP the subtype exclusion.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shoot the Sheriff");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target non-outlaw creature.".into(),
            // GAP: ObjectFilter has no "without any of these subtypes" predicate (outlaw = Assassin/Mercenary/Pirate/Rogue/Warlock).
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
