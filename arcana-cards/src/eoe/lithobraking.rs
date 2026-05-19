//! Lithobraking — `{2}{R}` instant, "Create a 1/1 colorless Lander artifact creature token.
//! Then you may sacrifice an artifact. When you do, Lithobraking deals 2 damage to each creature."
//!
//! GAP: Optional sacrifice of an artifact triggering damage to each creature (sacrifice as
//! part of resolution with triggered follow-up).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lithobraking");
    let _lander = reg.interner_mut().intern("Lander");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 1/1 colorless Lander artifact creature token. Then you may sacrifice an artifact. When you do, Lithobraking deals 2 damage to each creature.".into(),
                target_requirements: vec![],
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
    let lander = reg.interner().lookup("Lander").expect("Lander interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lander);
    let token = TokenDefinition {
        name: lander,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: optional sacrifice of artifact triggering deal 2 to each creature
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
