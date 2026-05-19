//! Firespout — `{2}{R/G}` sorcery. "Firespout deals 3 damage to each creature
//! without flying if {R} was spent to cast this spell and 3 damage to each
//! creature with flying if {G} was spent to cast this spell."
//!
//! # GAP: "if {R} was spent" / "if {G} was spent" — mana-spent tracking
//! during casting is not accessible in the resolver via the catalog's
//! `Effect::Conditional`; no condition variant inspects payment colors.
//! Best effort: deal 3 damage to each creature (non-flying wipe), omitting the
//! flying-selective conditional.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Firespout");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Firespout deals 3 damage to each creature without flying if {R} was spent to cast this spell and 3 damage to each creature with flying if {G} was spent to cast this spell.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mana-spent conditional (which colors were spent) not available in resolver
    // Best effort: deal 3 to each creature (treats as if both colors were spent)
    let filter = ObjectFilter::creature();
    let targets = script::ids_matching(state, &filter, entry.controller);
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: arcana_core::events::DamageTarget::Object(NULL_OBJECT_ID),
            amount: 3,
        }),
    }]
}
