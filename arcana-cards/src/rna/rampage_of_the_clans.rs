//! Rampage of the Clans — `{3}{G}` instant.
//! "Destroy all artifacts and enchantments. For each permanent destroyed this
//! way, its controller creates a 3/3 green Centaur creature token."
//!
//! # GAP: creating a token for each permanent's controller (not the spell
//! controller) is not expressible; all tokens go to entry.controller.
//! GAP: counting how many permanents were actually destroyed (some may be
//! indestructible) to gate token creation is not expressible. Token count
//! equals the count of matching permanents at resolution time.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rampage of the Clans");
    let _centaur = reg.interner_mut().intern("Centaur");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all artifacts and enchantments. For each permanent destroyed this way, its controller creates a 3/3 green Centaur creature token.".into(),
                target_requirements: vec![],
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
    // GAP: token per destroyed permanent's controller not supported; tokens go to entry.controller.
    let filter = ObjectFilter::new().with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT));
    let ids = script::ids_matching(state, &filter, entry.controller);
    let count = ids.len() as u32;
    let centaur = reg.interner().lookup("Centaur").expect("Centaur interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    let token = TokenDefinition {
        name: centaur,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    for _ in 0..count {
        effects.push(Effect::CreateToken { controller: entry.controller, token: token.clone() });
    }
    effects
}
