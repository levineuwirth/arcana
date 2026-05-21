//! Rapid Hybridization — `{U}` instant. "Destroy target creature. It
//! can't be regenerated. That creature's controller creates a 3/3
//! green Frog Lizard creature token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rapid Hybridization");
    let _frog = reg.interner_mut().intern("Frog");
    let _lizard = reg.interner_mut().intern("Lizard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature. It can't be regenerated. That creature's controller creates a 3/3 green Frog Lizard creature token.".into(),
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
    let frog = reg.interner().lookup("Frog").expect("Frog interned");
    let lizard = reg.interner().lookup("Lizard").expect("Lizard interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(lizard);
    let token = TokenDefinition {
        name: frog,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "It can't be regenerated" rider not in catalog.
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken {
            controller: script::target_controller(state, *id, entry.controller),
            token,
        },
    ]
}
