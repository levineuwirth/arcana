//! A-Excavation Explosion — `{2}{R}` instant. "Excavation Explosion
//! deals 4 damage to target creature or planeswalker. Create a tapped
//! Powerstone token."
//!
//! Planeswalker target and the Powerstone token's tapped state /
//! mana ability aren't catalog-expressible; the 4 damage to a
//! creature and a Powerstone artifact token are emitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Excavation Explosion");
    let _ps = reg.interner_mut().intern("Powerstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Excavation Explosion deals 4 damage to target creature or planeswalker. Create a tapped Powerstone token.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    let ps = reg.interner().lookup("Powerstone").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ps);
    let token = TokenDefinition {
        name: ps,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: planeswalker target; Powerstone tapped-state + mana ability.
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 4,
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
