//! Lost in the Spirit World — `{2}{U}` sorcery. "Return up to one target
//! creature to its owner's hand. Create a 1/1 colorless Spirit creature token
//! with 'This token can't block or be blocked by non-Spirit creatures.'"
//!
//! GAP: token ability "can't block or be blocked by non-Spirit creatures" —
//! no Effect variant for granting custom static abilities to tokens.
//! Token is created without that restriction.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lost in the Spirit World");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to one target creature to its owner's hand. Create a 1/1 colorless Spirit creature token with 'This token can't block or be blocked by non-Spirit creatures.'".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
        // GAP: "can't block or be blocked by non-Spirit creatures" static ability not expressible
    };
    let mut effects = Vec::new();
    if let Some(target) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnToHand { target: *id });
        }
    }
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
